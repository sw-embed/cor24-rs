# Changes

## 2026-03-16

- Add collapsible Instruction Trace panel to Web UI (last 100 entries)
- Record halt instruction (bra-to-self) in trace before halting
- Add `trace` command to cor24-dbg CLI debugger
- Convert all 14 assembler examples to reference as24-compatible syntax
- Translator emits decimal immediates instead of hex (as24 compat)
- Regenerate all 13 pipeline demos with decimal immediates
- Add Assert example with deliberate bug for debugging demo
- Add Loop Trace example demonstrating Run/Stop/Trace workflow
- Update Multiply example: native mul + loop, with assertions
- Add assembler range check for lc (0..127) and lcu (0..255)
- Document assembler compatibility analysis (docs/extended-assembler.md)

## 2026-03-15

- Add Comments example showing comment syntax and editability
- Fix Rust pipeline Compile step not scrolling to MSP430 assembly
- UART hex dump wraps at 8 bytes/line instead of one long line
- Remove max-height cap on code blocks in C/Rust pipeline tabs
- Fix stale load generation counter causing empty source panel
- Add load_generation prop to ProgramArea for re-select same example
- Fix Step dropdown misalignment in notebook debug cells
- Reorder tabs alphabetically: Assembler, C, Rust

## 2026-03-14

- Add C tab to web UI with CPipeline component and wizard
- Add Compile step to C pipeline wizard (Source → Compile → Assemble)
- Add Tutorial button to C pipeline sidebar
- Add C pipeline examples: fib, sieve (with printf runtime stubs)
- Add .comm directive and .byte fix to assembler
- Migrate assembler examples to jal calling convention
- Regenerate pipeline demos with jal calling convention
- Add instruction trace ring buffer and CLI --trace/--step modes
- Document Luther Johnson's COR24 calling convention feedback
