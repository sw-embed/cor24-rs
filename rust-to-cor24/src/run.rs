//! cor24-run: Simple CLI runner for COR24 assembly with LED output
//!
//! Usage: cor24-run [--step] <file.s>
//!        cor24-run --demo      # Run built-in LED blink demo

use std::collections::HashMap;
use std::env;
use std::fs;

/// Memory-mapped I/O address for LEDs
const IO_LEDSWDAT: u32 = 0xFF0000;

/// Minimal COR24 CPU state
struct Cpu {
    pc: u32,
    regs: [u32; 8],
    c: bool,
    mem: Vec<u8>,
    halted: bool,
    leds: u8,
    prev_leds: u8,
}

impl Cpu {
    fn new() -> Self {
        Self {
            pc: 0,
            regs: [0; 8],
            c: false,
            mem: vec![0; 65536],
            halted: false,
            leds: 0,
            prev_leds: 0,
        }
    }

    fn mask24(v: u32) -> u32 {
        v & 0xFFFFFF
    }

    fn sign_ext8(v: u8) -> u32 {
        if v & 0x80 != 0 {
            0xFFFF00 | (v as u32)
        } else {
            v as u32
        }
    }

    fn read_byte(&self, addr: u32) -> u8 {
        let addr = addr & 0xFFFFFF;
        if (addr & 0xFF0000) == 0xFF0000 {
            // I/O region - return 0 for switches (or could be configurable)
            0
        } else {
            self.mem[(addr as usize) % self.mem.len()]
        }
    }

    fn write_byte(&mut self, addr: u32, val: u8) {
        let addr = addr & 0xFFFFFF;
        if addr == IO_LEDSWDAT {
            self.leds = val;
        } else if (addr & 0xFF0000) != 0xFF0000 {
            let len = self.mem.len();
            self.mem[(addr as usize) % len] = val;
        }
    }

    fn read_word(&self, addr: u32) -> u32 {
        let b0 = self.read_byte(addr) as u32;
        let b1 = self.read_byte(addr.wrapping_add(1)) as u32;
        let b2 = self.read_byte(addr.wrapping_add(2)) as u32;
        b0 | (b1 << 8) | (b2 << 16)
    }

    fn write_word(&mut self, addr: u32, val: u32) {
        self.write_byte(addr, (val & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(1), ((val >> 8) & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(2), ((val >> 16) & 0xFF) as u8);
    }

    fn get_reg(&self, r: u8) -> u32 {
        self.regs[(r & 7) as usize] & 0xFFFFFF
    }

    fn set_reg(&mut self, r: u8, v: u32) {
        self.regs[(r & 7) as usize] = v & 0xFFFFFF;
    }

    /// Check if LEDs changed and print if so
    fn check_led_change(&mut self) {
        if self.leds != self.prev_leds {
            print_leds(self.leds);
            self.prev_leds = self.leds;
        }
    }

    /// Execute one instruction, returns true if should continue
    fn step(&mut self) -> bool {
        if self.halted {
            return false;
        }

        let byte0 = self.read_byte(self.pc);
        // Debug: uncomment to trace execution
        // eprintln!("PC={:04X} byte={:02X}", self.pc, byte0);

        // Halt check: 0x00 at address 0 or explicit halt
        if byte0 == 0x00 && self.pc == 0 {
            self.halted = true;
            return false;
        }

        // Simple decode based on opcode patterns
        // This is a minimal subset for the demo
        let (opcode, ra, rb) = decode_instruction(byte0);

        match opcode {
            0x09 => {
                // add ra,imm8
                let imm = self.read_byte(self.pc + 1);
                let val = Self::mask24(self.get_reg(ra).wrapping_add(Self::sign_ext8(imm)));
                self.set_reg(ra, val);
                self.pc = Self::mask24(self.pc + 2);
            }
            0x0B => {
                // la ra,imm24
                let b0 = self.read_byte(self.pc + 1) as u32;
                let b1 = self.read_byte(self.pc + 2) as u32;
                let b2 = self.read_byte(self.pc + 3) as u32;
                let imm24 = b0 | (b1 << 8) | (b2 << 16);
                if ra == 7 {
                    // jmp absolute
                    self.pc = imm24;
                } else {
                    self.set_reg(ra, imm24);
                    self.pc = Self::mask24(self.pc + 4);
                }
            }
            0x0E => {
                // lc ra,imm8
                let imm = self.read_byte(self.pc + 1);
                self.set_reg(ra, Self::sign_ext8(imm));
                self.pc = Self::mask24(self.pc + 2);
            }
            0x0F => {
                // lcu ra,imm8
                let imm = self.read_byte(self.pc + 1);
                self.set_reg(ra, imm as u32);
                self.pc = Self::mask24(self.pc + 2);
            }
            0x16 => {
                // sb ra,imm8(rb)
                let imm = self.read_byte(self.pc + 1);
                let addr = Self::mask24(self.get_reg(rb).wrapping_add(Self::sign_ext8(imm)));
                self.write_byte(addr, self.get_reg(ra) as u8);
                self.pc = Self::mask24(self.pc + 2);
            }
            0x17 => {
                // shl ra,rb
                let shift = self.get_reg(rb) & 0x1F;
                let val = Self::mask24(self.get_reg(ra) << shift);
                self.set_reg(ra, val);
                self.pc = Self::mask24(self.pc + 1);
            }
            0x08 => {
                // clu ra,rb
                self.c = self.get_reg(ra) < self.get_reg(rb);
                self.pc = Self::mask24(self.pc + 1);
            }
            0x03 => {
                // bra imm8
                let imm = self.read_byte(self.pc + 1);
                let next = Self::mask24(self.pc + 2);
                self.pc = Self::mask24(next.wrapping_add(Self::sign_ext8(imm)));
            }
            0x04 => {
                // brf imm8
                let imm = self.read_byte(self.pc + 1);
                let next = Self::mask24(self.pc + 2);
                if !self.c {
                    self.pc = Self::mask24(next.wrapping_add(Self::sign_ext8(imm)));
                } else {
                    self.pc = next;
                }
            }
            0x05 => {
                // brt imm8
                let imm = self.read_byte(self.pc + 1);
                let next = Self::mask24(self.pc + 2);
                if self.c {
                    self.pc = Self::mask24(next.wrapping_add(Self::sign_ext8(imm)));
                } else {
                    self.pc = next;
                }
            }
            _ => {
                // Unknown/halt
                self.halted = true;
                return false;
            }
        }

        self.check_led_change();
        true
    }

    fn load_program(&mut self, data: &[u8]) {
        for (i, &b) in data.iter().enumerate() {
            self.mem[i] = b;
        }
    }
}

/// Decode instruction byte to (opcode, ra, rb)
fn decode_instruction(byte: u8) -> (u8, u8, u8) {
    // Simplified decode for common patterns
    match byte {
        // add ra,imm (0x09, 0x0A, 0x0B for r0,r1,r2)
        0x09 => (0x09, 0, 0),
        0x0A => (0x09, 1, 0),
        0x0B => (0x09, 2, 0),
        // la ra,imm24 (0x29-0x2F)
        0x29 => (0x0B, 0, 0),
        0x2A => (0x0B, 1, 0),
        0x2B => (0x0B, 2, 0),
        0x2C => (0x0B, 3, 0),
        0x2D => (0x0B, 4, 0),
        0x2E => (0x0B, 5, 0),
        0x2F => (0x0B, 6, 0),
        0xC7 => (0x0B, 7, 0), // jmp addr
        // lc ra,imm (0x44-0x4B)
        0x44 => (0x0E, 0, 0),
        0x45 => (0x0E, 1, 0),
        0x46 => (0x0E, 2, 0),
        0x47 => (0x0E, 3, 0),
        // lcu ra,imm (0x48-0x4F)
        0x48 => (0x0F, 0, 0),
        0x49 => (0x0F, 1, 0),
        0x4A => (0x0F, 2, 0),
        0x4B => (0x0F, 3, 0),
        // sb ra,imm(rb) - common patterns
        0x82 => (0x16, 0, 1), // sb r0,(r1)
        0x83 => (0x16, 0, 2),
        0x84 => (0x16, 1, 0),
        0x85 => (0x16, 1, 1),
        0x86 => (0x16, 1, 2),
        0x87 => (0x16, 2, 0),
        // shl ra,rb
        0x88 => (0x17, 0, 0),
        0x89 => (0x17, 0, 1),
        0x8A => (0x17, 0, 2),
        0x8E => (0x17, 2, 0),
        // clu ra,rb
        0xCE => (0x08, 5, 0), // clu z,r0
        0xCF => (0x08, 5, 1),
        0xD0 => (0x08, 5, 2),
        0x20 => (0x08, 0, 2), // clu r0,r2
        0x21 => (0x08, 2, 0), // clu r2,r0
        // branches
        0x13 => (0x03, 0, 0), // bra
        0x14 => (0x04, 0, 0), // brf
        0x15 => (0x05, 0, 0), // brt
        _ => (0xFF, 0, 0),    // unknown
    }
}

/// Print LED state
fn print_leds(leds: u8) {
    print!("LEDs: ");
    for i in (0..8).rev() {
        if (leds >> i) & 1 == 1 {
            print!("●");
        } else {
            print!("○");
        }
    }
    println!("  (0x{:02X})", leds);
}

/// Simple assembler for demo
fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    let mut fixups: Vec<(usize, String, bool)> = Vec::new(); // (offset, label, is_branch)

    // First pass: collect labels
    let mut addr = 0u32;
    for line in source.lines() {
        let line = line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        if let Some(label) = line.strip_suffix(':') {
            labels.insert(label.trim().to_string(), addr);
            continue;
        }

        // Estimate instruction size
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let mnemonic = parts[0].to_lowercase();
        addr += match mnemonic.as_str() {
            "la" => 4,
            "halt" => 4,
            "add" | "lc" | "lcu" | "sb" | "lb" | "bra" | "brt" | "brf" => 2,
            _ => 1,
        };
    }

    // Second pass: generate code
    for line in source.lines() {
        let line = line.split(';').next().unwrap_or("").trim();
        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let mnemonic = parts[0].to_lowercase();
        let operands = if parts.len() > 1 {
            parts[1].split(',').map(|s| s.trim()).collect::<Vec<_>>()
        } else {
            vec![]
        };

        match mnemonic.as_str() {
            "lc" => {
                let ra = parse_reg(&operands[0])?;
                let imm = parse_imm(&operands[1])?;
                output.push(0x44 + ra);
                output.push(imm as u8);
            }
            "lcu" => {
                let ra = parse_reg(&operands[0])?;
                let imm = parse_imm(&operands[1])?;
                output.push(0x48 + ra);
                output.push(imm as u8);
            }
            "la" => {
                let ra = parse_reg(&operands[0])?;
                let imm = parse_imm24(&operands[1], &labels)?;
                output.push(0x29 + ra);
                output.push((imm & 0xFF) as u8);
                output.push(((imm >> 8) & 0xFF) as u8);
                output.push(((imm >> 16) & 0xFF) as u8);
            }
            "sb" => {
                let ra = parse_reg(&operands[0])?;
                let (offset, rb) = parse_mem_operand(&operands[1])?;
                // Simplified: sb r0,0(r1) -> 0x82 0x00
                output.push(0x82 + ra * 3 + rb);
                output.push(offset as u8);
            }
            "shl" => {
                let ra = parse_reg(&operands[0])?;
                let rb = parse_reg(&operands[1])?;
                output.push(0x88 + ra * 3 + rb);
            }
            "clu" => {
                let ra = parse_reg(&operands[0])?;
                let rb = parse_reg(&operands[1])?;
                if ra == 5 {
                    // clu z,rx
                    output.push(0xCE + rb);
                } else {
                    output.push(0x20 + ra); // simplified
                }
            }
            "bra" => {
                output.push(0x13);
                let target = operands[0];
                if let Some(&addr) = labels.get(target) {
                    let current = output.len() as i32;
                    let offset = (addr as i32) - current - 1;
                    output.push(offset as u8);
                } else {
                    fixups.push((output.len(), target.to_string(), true));
                    output.push(0);
                }
            }
            "brf" => {
                output.push(0x14);
                let target = operands[0];
                if let Some(&addr) = labels.get(target) {
                    let current = output.len() as i32;
                    let offset = (addr as i32) - current - 1;
                    output.push(offset as u8);
                } else {
                    fixups.push((output.len(), target.to_string(), true));
                    output.push(0);
                }
            }
            "brt" => {
                output.push(0x15);
                let target = operands[0];
                if let Some(&addr) = labels.get(target) {
                    let current = output.len() as i32;
                    let offset = (addr as i32) - current - 1;
                    output.push(offset as u8);
                } else {
                    fixups.push((output.len(), target.to_string(), true));
                    output.push(0);
                }
            }
            "halt" => {
                // la ir,0 (jmp to address 0)
                output.push(0xC7);
                output.push(0);
                output.push(0);
                output.push(0);
            }
            _ => return Err(format!("Unknown instruction: {}", mnemonic)),
        }
    }

    // Apply fixups
    for (offset, label, is_branch) in fixups {
        if let Some(&addr) = labels.get(&label) {
            if is_branch {
                let current = offset as i32;
                let rel = (addr as i32) - current - 1;
                output[offset] = rel as u8;
            }
        } else {
            return Err(format!("Undefined label: {}", label));
        }
    }

    Ok(output)
}

fn parse_reg(s: &str) -> Result<u8, String> {
    match s.to_lowercase().as_str() {
        "r0" => Ok(0),
        "r1" => Ok(1),
        "r2" => Ok(2),
        "r3" | "fp" => Ok(3),
        "r4" | "sp" => Ok(4),
        "r5" | "z" => Ok(5),
        "r6" | "iv" => Ok(6),
        "r7" | "ir" => Ok(7),
        _ => Err(format!("Invalid register: {}", s)),
    }
}

fn parse_imm(s: &str) -> Result<i32, String> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i32::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else if s.starts_with('-') {
        s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
    } else {
        s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
    }
}

fn parse_imm24(s: &str, _labels: &HashMap<String, u32>) -> Result<u32, String> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else {
        s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
    }
}

fn parse_mem_operand(s: &str) -> Result<(i32, u8), String> {
    // Parse "offset(reg)" or "(reg)"
    if let Some(paren) = s.find('(') {
        let offset_str = &s[..paren];
        let reg_str = s[paren + 1..].trim_end_matches(')');
        let offset = if offset_str.is_empty() {
            0
        } else {
            parse_imm(offset_str)?
        };
        let rb = parse_reg(reg_str)?;
        Ok((offset, rb))
    } else {
        Err(format!("Invalid memory operand: {}", s))
    }
}

// Pre-assembled LED counter program
// Simplest demo: count 0,1,2,3... and display on LEDs
// Uses only: la, lc, sb, add imm, bra
const DEMO_BYTES: &[u8] = &[
    // 0x00: la r1,0xFF0000    (4 bytes) - LED address
    0x2A, 0x00, 0x00, 0xFF,
    // 0x04: lc r0,0           (2 bytes) - counter = 0
    0x44, 0x00,
    // 0x06: loop: sb r0,0(r1) (2 bytes) - write to LEDs
    0x82, 0x00,
    // 0x08: add r0,1          (2 bytes) - counter++
    0x09, 0x01,
    // 0x0A: bra loop          (2 bytes) - back to loop
    // next_pc = 0x0C, target = 0x06, offset = 0x06 - 0x0C = -6 = 0xFA
    0x13, 0xFA,
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let use_demo = args.len() < 2 || args[1] == "--demo";
    let step_mode = args.contains(&"--step".to_string());

    let bytes: Vec<u8> = if use_demo {
        println!("=== COR24 LED Counter Demo ===\n");
        println!("Program:");
        println!("        la   r1, 0xFF0000  ; LED I/O address");
        println!("        lc   r0, 0         ; counter = 0");
        println!("loop:");
        println!("        sb   r0, 0(r1)     ; Write to LEDs");
        println!("        add  r0, 1         ; counter++");
        println!("        bra  loop          ; repeat forever\n");
        DEMO_BYTES.to_vec()
    } else {
        let filename = if step_mode && args.len() > 2 {
            &args[2]
        } else {
            &args[1]
        };
        let source = fs::read_to_string(filename).expect("Failed to read file");
        match assemble(&source) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Assembly error: {}", e);
                std::process::exit(1);
            }
        }
    };

    println!("Loaded {} bytes\n", bytes.len());

    // Run
    let mut cpu = Cpu::new();
    cpu.load_program(&bytes);

    println!("Running...\n");
    print_leds(0); // Initial state

    let max_steps = 100;
    let mut steps = 0;

    while cpu.step() && steps < max_steps {
        steps += 1;
        if step_mode {
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }

    println!("\nCompleted {} steps", steps);
    if cpu.halted {
        println!("CPU halted");
    }
}
