# Feedback Checklist

Tracks feedback from Luther Johnson and how each item was addressed.

## Architecture Corrections (feedback1.txt, cor24-feedback.txt)

- [x] Address space is 24 bits (16 MB), not 64 KB — fixed in docs and code
- [x] Only r0, r1, r2 are true GP registers (not 8) — fixed in ISA Ref, Tutorial
- [x] Z register is zero-only for compare instructions — fixed in ISA Ref, Tutorial
- [x] Instructions are 1, 2, or 4 bytes (never 3) — fixed in ISA Ref, Tutorial
- [x] PC-relative branch: PC is 2 ahead at execute time — implemented in executor

## Branding (research/feedback1.txt, research/feedback2a.txt)

- [x] Refer to CPU as "MakerLisp COR24" (mention once) — in header and README
- [x] Use "COR24" for subsequent references — throughout UI and docs
- [ ] Luther review for compatibility — pending his review

## Hardware Corrections (research/feedback2b.txt)

- [x] FPGA is MachXO (not MachXO2) — corrected in docs
- [x] 211 instruction forms (32 operations x register fields) — in ISA Ref
- [x] C = condition flag (not carry) — fixed everywhere
- [x] Only R0-R2 are GP; R3-R7 are special purpose — in ISA Ref, Tutorial
- [x] R5 is always-zero, accessed as "z" in compares only — in ISA Ref
- [x] 1 MB on-board SRAM — in memory map
- [x] Stack: FEE000-FEFFFF (8 KB range, 3 KB populated), SP init FEEC00 — in ISA Ref
- [x] Reading UART data register auto-acknowledges RX — in emulator (read_byte_exec)
- [x] D2 = user LED (active-low pull-up), S2 = button (active-low pull-up) — in emulator
- [ ] Two separate adders (add24ci, add24cico) — not modeled (functional emulation only)
- [ ] Test-bench simulates entire SOC — out of scope for web emulator

## Calling Convention (research/20260314-cor24-feedback.txt, research/20260303-luther-notes.txt)

- [x] Use jal instruction for calls (not manual la/push/jmp) — all examples updated
- [x] jal stores return address in r1 — documented in ISA Ref, Tutorial
- [x] Save/restore convention: push fp, r2, r1 / mov fp,sp — in Tutorial, Help
- [x] Arguments on stack, return value in r0 — documented
- [x] Leaf routines can skip unused saves — noted in Tutorial
- [x] Interrupts detected on bra/brf/brt/jmp/jal — implemented in executor

## NOP Instruction (cor24-nop-email.txt)

- [x] NOP is opcode 0xFF (actually "zxt z,z") — assembler emits 0xFF
- [x] "add r0,r0" is NOT a no-op (doubles r0) — fixed, was the old encoding
- [x] Executor handles 0xFF as true no-op — implemented

## Assembler Syntax (cor24-nop-email.txt)

- [x] Support Intel hex notation (0FFh not 0xFF) — parse_number supports both
- [x] sxt and zxt are supported — in assembler
- [x] sub sp,dddddd is supported — in assembler
- [x] la is supported by reference assembler — confirmed, examples use it
- [x] Labels must be on own line (reference compat) — all examples fixed

## UART TX Optimization (cor24-fib-email.txt)

- [x] Use cls r2,z / brt for TX busy check — 3 examples updated
- [x] Bit 7 on status byte = sign bit after lb — leveraged in all UART examples
- [x] Applied to C pipeline examples too — fib.cor24.s, sieve.cor24.s

## I/O Hardware Details (research/feedback3.txt)

- [x] D1 = power LED, D2 = user LED — D2 in emulator
- [x] S2 = momentary button (normal high, pressed low) — in emulator
- [x] No halt instruction, use bra to self — all examples use this pattern
- [x] Never use r3-r5 as GP registers — enforced in examples
- [x] Memory layout: code at 0, I/O at 0xFF0000+ — in ISA Ref memory map
- [ ] More examples needed for various features — ongoing
- [ ] NOP = jmp -2 (alternative) — not implemented, using 0xFF instead
- [ ] HALT = jmp -4 — not implemented, using bra-to-self instead

## Documentation Updates (feedback1-fixes-plan.md)

- [x] Phase 1: Documentation files — completed
- [x] Phase 2: Source code comments — completed
- [x] Phase 3: Work documents — completed
- [x] Phase 4: rust-to-cor24 pipeline — completed
- [x] Phase 5: Rebuild and verify — completed
