# Rust to COR24 Pipeline

Compiles Rust to COR24 machine code via the MSP430 target as a 16-bit intermediate.

## Pipeline Overview

```
                      ┌─────────┐
  demo_blinky.rs      │  rustc  │   Uses MSP430 as 16-bit backend
  (Rust source)  ───► │ nightly │   (no custom compiler needed)
                      │ msp430  │
                      └────┬────┘
                           │
                    ┌──────▼───────┐
                    │  .s file     │   MSP430 assembly text
                    │  (--emit asm)│   Functions in alphabetical section order
                    └──────┬───────┘
                           │
                 ┌─────────▼──────────┐
                 │  msp430-to-cor24   │   Translator (this project)
                 │  --entry demo_blinky│   Adds reset vector prologue
                 └─────────┬──────────┘
                           │
                    ┌──────▼───────┐
                    │  .cor24.s    │   COR24 assembly text
                    │              │   With `bra demo_blinky` at address 0
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  COR24       │   Two-pass assembler (in cor24-emulator crate)
                    │  Assembler   │   Resolves labels, encodes instructions
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  Binary blob │   Raw bytes, loaded at address 0x000000
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  COR24 CPU   │   PC starts at 0x000000 (RESET_ADDRESS)
                    │  Emulator    │   Executes `bra demo_blinky` first
                    └──────────────┘
```

## How the Entry Point Works

### The Problem

The Rust compiler (`rustc`) emits functions in `.section .text.<name>` sections,
ordered **alphabetically by function name** — not by source order. So a file with
`demo_blinky`, `delay`, and `mmio_write` produces MSP430 assembly with sections in
this order:

```
.section .text._RN...rust_begin_unwind   ← panic handler (first alphabetically!)
.section .text.delay
.section .text.demo_blinky               ← our entry point (buried in the middle)
.section .text.mmio_write
```

The COR24 CPU starts executing at address 0x000000. Without intervention, it would
execute the panic handler (an infinite loop), not our demo.

### The Solution: Reset Vector Prologue

The `msp430-to-cor24` translator emits a **branch instruction at address 0** that
jumps to the entry point function:

```asm
; Reset vector -> demo_blinky
    bra     demo_blinky          ; 4 bytes at address 0x000000

; --- function: _RN...rust_begin_unwind ---   (panic handler)
_RN...rust_begin_unwind:
.LBB0_1:
    bra     .LBB0_1              ; infinite loop (never reached in normal execution)

; --- function: delay ---
delay:
    ...

; --- function: demo_blinky ---              ; ← CPU jumps here from address 0
demo_blinky:
    la      r0, 0xFF0000
    lc      r1, 1
    ...
```

This mirrors how real microcontrollers work: the hardware reset vector at address 0
contains a jump to the startup code.

### How the Entry Point is Identified

1. **Explicit `--entry <func>` flag** (preferred for multi-function files):
   ```bash
   msp430-to-cor24 demos.msp430.s --entry demo_blinky -o demo_blinky.cor24.s
   ```

2. **Auto-detection** from `.globl` directives in the MSP430 assembly:
   - First: looks for any symbol starting with `demo_` or named `main`
   - Fallback: first non-mangled, non-helper symbol
   - Skips: mangled names (`_RN...`), known helpers (`mmio_write`, `delay`, etc.)

3. **No `.globl` directives** (e.g., hand-written single-function tests):
   - No prologue emitted — first instruction is at address 0 (legacy behavior)

### In the Rust Source

The entry point is the function marked `#[no_mangle]` with a `demo_` prefix:

```rust
#[no_mangle]                    // prevents name mangling → linker-visible symbol
pub unsafe fn demo_blinky() -> ! {
    loop {
        mmio_write(LED_ADDR, 1);
        delay(5000);
        mmio_write(LED_ADDR, 0);
        delay(5000);
    }
}
```

The `#[no_mangle]` attribute makes the function visible as `.globl demo_blinky` in
the MSP430 assembly output. The translator then recognizes it as the entry point.

## Register Mapping

| MSP430 | COR24 | Role |
|--------|-------|------|
| r12 | r0 | arg0 / return value |
| r13 | r1 | arg1 |
| r14 | r2 | arg2 |
| r1 | sp | stack pointer |
| r4-r11 | stack | spilled to fp-relative offsets |

## I/O Address Mapping

MSP430 uses 16-bit addresses. COR24 uses 24-bit. The translator maps:

| MSP430 (16-bit) | COR24 (24-bit) | Device |
|------------------|-----------------|--------|
| 0xFF00 | 0xFF0000 | LED D2 / Button S2 |
| 0xFF01 | 0xFF0100 | UART data register |
| 0xFF02 | 0xFF0101 | UART status register |

In Rust source, we define 16-bit constants that the MSP430 target can handle:
```rust
const LED_ADDR: u16 = 0xFF00;     // → 0xFF0000 after translation
const UART_DATA: u16 = 0xFF01;    // → 0xFF0100 after translation
```

## CLI Usage

```bash
# Translate a single MSP430 .s file to COR24 assembly
msp430-to-cor24 input.msp430.s --entry demo_blinky -o output.cor24.s

# Compile a Rust project end-to-end
msp430-to-cor24 --compile path/to/rust/project --entry demo_blinky

# Run built-in test case (no entry prologue needed — single functions)
msp430-to-cor24 --test
```

### Compile mode prerequisites

```bash
rustup toolchain install nightly
rustup target add msp430-none-elf --toolchain nightly
```

## File Types in the Pipeline

| Extension | Format | Description |
|-----------|--------|-------------|
| `.rs` | Rust source | `#![no_std]`, `#[panic_handler]`, `#[no_mangle]` on entry |
| `.msp430.s` | MSP430 asm text | Output of `rustc --emit asm`, `.section .text.<func>` per function |
| `.cor24.s` | COR24 asm text | `bra <entry>` prologue + translated instructions + labels |
| (in-memory) | `Vec<u8>` | Raw bytes from COR24 assembler, loaded at address 0 |

## Project Structure

```
rust-to-cor24/
├── src/
│   ├── lib.rs           # Re-exports translate_msp430
│   ├── msp430.rs        # MSP430 parser + COR24 translator + entry point detection
│   ├── msp430_cli.rs    # CLI: --entry, -o, --compile, --test
│   ├── run.rs           # COR24 emulator runner (assembles + executes)
│   └── pipeline.rs      # WASM pipeline (legacy, unused)
├── output/              # Pre-generated demo outputs
│   ├── demo_blinky.msp430.s   # MSP430 asm from rustc
│   ├── demo_blinky.cor24.s    # COR24 asm with bra prologue
│   └── demo_blinky.log        # Emulator run output
└── data/
    └── pipeline/
        └── demos.rs     # Complete Rust source for all demos
```

## Conventions

1. **Entry point naming**: Functions named `demo_*` or `main` are recognized as entry points
2. **`#[no_mangle]`**: Required on entry point and all functions called across translation units
3. **`#[inline(never)]`**: Required on helper functions to prevent inlining (which would eliminate the callable function)
4. **`#![no_std]` + `#[panic_handler]`**: Required — no standard library on bare metal
5. **Reset vector**: The `bra <entry>` at address 0 is the COR24 equivalent of a hardware reset vector
6. **Halt convention**: `loop {}` in Rust compiles to a self-branch (`bra .`), detected by the emulator as halt
