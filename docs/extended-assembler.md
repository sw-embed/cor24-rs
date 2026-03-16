# Extended Assembler: Compatibility with Reference cor24 Assembler

## Background

The COR24 emulator includes a Rust-based assembler that supports all standard
cor24 instructions plus several convenience extensions. The reference cor24
assembler (`as24`, running as `asld24-server` on queenbee:7412) does not
support these extensions.

We want hand-written `.s` examples in the Web UI and CLI to be valid for
**both** assemblers, so a viewer could use either tool.

## Validation Results

Instruction-by-instruction encoding comparison shows **100% agreement** on all
85 instruction encodings that both assemblers support. The Rust assembler
produces identical bytes for every standard instruction.

One minor difference: `nop` — ours emits 1 byte (`0x00` = `add r0,r0`),
reference emits 3 bytes (`00 00 00`).

## Extensions Our Assembler Supports (Reference Does Not)

### 1. `la` — Load Address (4-byte instruction)

**This is a real COR24 hardware instruction** with opcode `0x0B` in the decode
ROM extracted from `cor24_cpu.v`. It loads a 24-bit immediate into a register.
The reference assembler simply does not implement it yet. There is no
multi-instruction equivalent — `la` is the only way to load an arbitrary
24-bit value into a register in one instruction.

Encoding: 4 bytes (opcode + 24-bit LE immediate).
- `la r0, addr` → `0x29, lo, mid, hi`
- `la r1, addr` → `0x2A, lo, mid, hi`
- `la r2, addr` → `0x2B, lo, mid, hi`

**Impact:** ALL 12 assembler examples and ALL 12 pipeline examples use `la`.
Without `la`, programs that reference 24-bit addresses (I/O registers at
`0xFF0000`, UART at `0xFF0100`, function calls via label) cannot be written.

### 2. Inline Labels (`label: instruction`)

Our assembler allows `halt: bra halt` on one line. The reference requires
the label on its own line:

```
; Our syntax (extended):
halt: bra halt

; Reference syntax:
halt:
        bra halt
```

**Impact:** ALL 12 assembler examples use inline labels.

### 3. `#` Comments

Our assembler accepts both `;` and `#` as comment characters. The reference
only accepts `;`.

**Impact:** 1 file (`comments.s`).

### 4. Hex Immediates in Standard Instructions

`lc r0, 0x3F` works in our assembler. The reference requires decimal: `lc r0, 63`.
Note: hex in `la` operands is moot since `la` itself is unsupported.

**Impact:** 1 file (`echo.s`) uses hex in `lc`/`lcu` operands.

### 5. Other Extensions (unused in examples)

- `sxt`, `zxt` (sign/zero extend) — not used in any example
- `sub rX, imm` on GP registers — not used (only `sub sp, imm` which is standard)

## File-by-File Audit

### Assembler Examples (`src/examples/assembler/`)

| File | `la` | Inline labels | `#` comments | Hex in lc/lcu |
|---|---|---|---|---|
| `add.s` | 1 | 1 | - | - |
| `blink_led.s` | 1 | 1 | - | - |
| `button_echo.s` | 1 | 1 | - | - |
| `comments.s` | - | 1 | 2 | - |
| `countdown.s` | 1 | 1 | - | - |
| `echo.s` | 5 | - | - | 5 |
| `fibonacci.s` | 6 | 2 | - | - |
| `memory_access.s` | 4 | 1 | - | - |
| `multiply.s` | 4 | 2 | - | - |
| `nested_calls.s` | 4 | 2 | - | - |
| `stack_variables.s` | 3 | 1 | - | - |
| `uart_hello.s` | 7 | 2 | - | - |

### Pipeline Examples (`src/examples/rust_pipeline/*.cor24.s`)

All 12 files use `la` extensively (5-30+ occurrences each). These are compiler
output from the Rust→MSP430→COR24 pipeline and are not intended to be
hand-editable or reference-assembler-compatible.

## Proposed Solution: Macro Preprocessor

### Design

Add a preprocessor pass that expands macros **before** the assembler pass.
The preprocessor operates on text, emitting standard `.s` syntax. This is a
single pass — no iteration needed because macros expand to fixed instruction
sequences with known sizes.

```
Source (.sx)  →  Preprocessor  →  Standard .s  →  Assembler  →  Binary
```

### Macro Definitions

**`LA` macro** — expands `la rX, value` to the 4-byte instruction encoding
inline, using `.byte` directives:

```
; Input:
        la r0, 0xFF0100

; Preprocessor output:
        .byte 0x29, 0x00, 0x01, 0xFF
```

This works because `.byte` emits raw bytes and the assembler already supports
it. The 4 bytes are the exact encoding of the `la` instruction (opcode byte
for the destination register + 24-bit LE address). Since the expansion is a
fixed 4 bytes, all subsequent branch offsets are correct without multiple
passes.

**Limitation:** `la rX, label` (with a forward reference) cannot be expanded
by a text preprocessor because the label's address is not known yet. Options:
- Require labels used with `la` to be defined before use (backward refs only)
- Two-pass preprocessor: first pass collects label addresses, second expands
- Accept that `la` with labels requires the extended assembler

**Inline label macro** — expands `label: instruction` to two lines:

```
; Input:
halt: bra halt

; Preprocessor output:
halt:
        bra halt
```

This is a trivial text transformation with no size implications.

**Comment normalization** — replace `#` with `;`.

**Hex-to-decimal** — convert `0xNN` immediates in standard instructions to
decimal.

### File Extensions

| Extension | Meaning |
|---|---|
| `.s` | Standard cor24 assembly, compatible with reference `as24` |
| `.sx` | Extended assembly with macros (`la`, inline labels, etc.) |
| `.cor24.s` | Compiler-generated pipeline output (extended, not hand-edited) |

### Implementation Plan

1. **Phase 1: Fix examples** — Rewrite the 12 hand-written assembler examples
   to split inline labels onto their own lines and replace `#` comments with
   `;`. Keep `la` (it's a real instruction the reference should support).
   Convert hex immediates in `lc`/`lcu` to decimal.

2. **Phase 2: Preprocessor** — Implement a text-based preprocessor that
   expands `.sx` files to `.s`. Gate extended features behind this step.

3. **Phase 3: Assembler `--strict` flag** — Add a mode that rejects any
   syntax the reference assembler wouldn't accept, for validation.

### Open Questions

- Should we lobby for `la` support in the reference assembler? It's a real
  hardware instruction. The reference assembler is simply missing it.
- Should inline labels be considered an error in strict mode, or just a
  warning? They're purely a syntax convenience with no semantic difference.
- For `la rX, label`, the preprocessor cannot resolve forward references.
  Should we accept this limitation or implement a two-pass preprocessor?
