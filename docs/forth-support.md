# COR24 Forth Support — Assembler & ISA Findings

Investigation of COR24 assembler capabilities and ISA constraints relevant to
building a DTC (Direct Threaded Code) Forth targeting `cor24-run`.

All findings verified against the **reference `as24` assembler** (local build
from `docs/research/asld24/as24.c`) and the emulator source code.

## 1. `.word label_name` — label address emission

**Reference as24: YES, fully supported (forward and backward references).**

```
; Backward reference
start:
  mov r0,r1
  .word start         ; emits 00 00 00 (address of start)

; Forward reference
  .word fwd_label     ; emits correct address (resolved by linker)
  mov r0,r1
fwd_label:
  nop
```

Both cases correctly emit the label's 24-bit address as a little-endian 3-byte
word. The reference assembler uses a `FIXDD24` fixup type for symbol references,
resolved during linking.

**One label per `.word` directive.** The reference as24 implementation
(`as24.c:692`) first tries to parse all tokens as numeric (`scani24`). If any
token fails numeric parse, it falls through to `FIXDD24` handling which stores
only the **first** non-numeric token as a symbol reference (length=3). This means:

```
.word 42, 100, 200     ; OK — multiple numeric values on one line
.word my_label         ; OK — single symbol reference
.word lab1, lab2       ; BAD — only lab1 captured, lab2 lost
.word 42, lab1         ; BAD — 42 parsed, then lab1 triggers single-symbol path
```

For DTC Forth parameter fields, use one `.word` per line:

```
; Colon definition for DOUBLE: dup +
DOUBLE_pfa:
  .word xt_dup
  .word xt_plus
  .word xt_exit
```

**Our assembler (`src/assembler.rs`): NO — `.word` only accepts numeric
literals.** The `.word` handler (line 231) calls `parse_number()`, which does not
resolve labels or create forward references. `.word label_name` silently emits
nothing (no error, no bytes).

**Action required:** Extend our assembler's `.word` to resolve labels. This needs:
- Label lookup in the first pass (backward refs)
- A new `ForwardRef` with `RefType::Absolute24` for unresolved labels (forward refs)
- Follow the one-label-per-directive convention from reference as24
- ~20 lines of code, following the existing `la` label resolution pattern

This is critical for DTC Forth — every colon definition's parameter field is a
list of code field addresses.

## 2. `.byte` for name strings

**Reference as24: YES.** Emits raw bytes from comma-separated numeric values.

```
.byte 72,101,108,108,111    ; "Hello" as ASCII bytes
```

Verified in both reference as24 and our assembler. Our assembler also supports
`.byte` with comma-separated values (`assembler.rs:217`), with tests at line
1261.

There is no string literal syntax (`.ascii`/`.asciz` are not supported in either
assembler). Encode ASCII as numeric values.

## 3. Word alignment

**Misaligned `lw`/`sw` works correctly in the emulator.** The implementation
(`state.rs:526`) does three individual `read_byte` calls — no alignment check:

```rust
pub fn read_word(&self, addr: u32) -> u32 {
    let b0 = self.read_byte(addr) as u32;
    let b1 = self.read_byte(addr.wrapping_add(1)) as u32;
    let b2 = self.read_byte(addr.wrapping_add(2)) as u32;
    b0 | (b1 << 8) | (b2 << 16)
}
```

**Neither assembler supports `.align`.** Reference as24 rejects it:
```
? Line 2: unknown instruction/directive: '.align 3'
```
Our assembler parses but ignores it (no-op at `assembler.rs:262`).

For dictionary entries, manually pad name fields with extra `.byte 0` values to
maintain 3-byte alignment of the code field. Misalignment won't break anything
in the emulator, but keeping words aligned makes memory dumps easier to read and
matches the real hardware's preferred access pattern.

## 4. Stack size

Default EBR stack is 3 KB (SP starts at `0xFEEC00`, 1024 cells of 3 bytes
each). With `--stack-kilobytes 8`, SP starts at `0xFF0000`.

For Forth with separate data and return stacks:
- **Return stack** (hardware SP): used for `>R`, `R>`, DO/LOOP counters, and
  nested calls. Typical depth is modest — 64-128 cells handles deep nesting.
- **Data stack** (software-managed, e.g. via fp or a GP register): parameter
  stack for Forth values. Rarely exceeds 32-64 cells in normal operation.

**Recommendation:** Start with 3 KB. That's 1024 cells — more than enough for
both stacks even if they share the space. Only bump to 8 KB if you're doing
something unusual (large local buffers, deep recursion). The 8 KB option exists
via `--stack-kilobytes 8` if needed.

## 5. Immediate arithmetic on registers

### `add ra,imm8` (2-byte instruction, signed -128..+127)

Supported for: **r0, r1, r2, sp** — verified in encode table (`isa/src/encode.rs:28`):

| Register | Opcode byte |
|----------|-------------|
| r0       | 0x09        |
| r1       | 0x0A        |
| r2       | 0x0B        |
| sp       | 0x0C        |

**fp is NOT supported.** Reference as24 confirms:
```
? Line 1: unknown instruction/directive: 'add fp -3'
```

### `sub sp,imm24` (4-byte instruction)

Opcode 0xA2 — takes a 24-bit immediate. Only works on sp.

### Implications for Forth data stack pointer

If using fp as the data stack pointer, you cannot do `add fp,-3` to push a cell.
Options:

1. **Use sp for data stack, software-manage return stack** — `add sp,-3` works
   (2 bytes). Return stack uses `push`/`pop` or a dedicated memory region with
   a GP register as pointer.

2. **Use a GP register (r2) as data stack pointer** — `add r2,-3` works (2
   bytes). Costs one of only three GP registers permanently.

3. **Use fp with register arithmetic** — `mov r0,-3` + `add fp,r0` — works but
   costs 3 bytes and clobbers r0.

Option 1 is the most natural for COR24: the hardware stack (sp + push/pop) is
the data stack, and the return stack lives in a separate memory region managed
by a dedicated register.

## Summary of assembler gaps

| Feature | Reference as24 | Our assembler | Action |
|---------|---------------|---------------|--------|
| `.word label` | Yes (FIXDD24) | No (numeric only) | **Must add** |
| `.byte` values | Yes | Yes | None |
| `.align` | No | No (ignored) | N/A |
| `.comm` | Yes | Yes | None |
| `.ascii`/`.asciz` | No | No | N/A |
