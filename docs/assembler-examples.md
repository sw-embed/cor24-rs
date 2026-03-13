# COR24 Assembler Examples

Hand-written COR24 assembly programs for learning the instruction set.
These are available in the **Assembler tab** of the
[web emulator](https://sw-embed.github.io/cor24-rs/) (click "Examples" in
the sidebar) and can also be run from the command line with `cor24-dbg`.

## Web UI

1. Open the [web emulator](https://sw-embed.github.io/cor24-rs/)
2. Click **Examples** in the sidebar and pick a program
3. Click **Assemble** to load it into memory
4. Click **Run** (or **Step** to single-step)

## CLI

The `scripts/` directory has shell scripts that build `cor24-dbg` and run
pre-assembled `.lgo` programs:

```bash
scripts/demo-cli-hello-world.sh   # UART "Hello, World!" output
scripts/demo-cli-count-down.sh    # Countdown with breakpoint debugging
scripts/demo-cli-led-blink.sh     # LED D2 blink with UART logging
scripts/demo-cli-sieve.sh         # Sieve of Eratosthenes benchmark
```

You can also run any `.s` file interactively with the debugger:

```bash
cargo build -p cor24-cli
./target/debug/cor24-dbg tests/programs/hello_world.lgo
# (cor24) run 1000
# (cor24) uart
# (cor24) quit
```

## Example Catalog

### Arithmetic & Logic

| Example | Description |
|---------|-------------|
| **Add** | Compute `100 + 200 + 42 = 342`, store result to memory at 0x0100. |
| **Multiply** | `6 × 7 = 42` via repeated-addition loop, print result to UART. |
| **Stack Variables** | Local variables via push/pop register spilling. Computes `a=seed+1, b=a+seed, c=b+a, result=a^b^c`. |

### Control Flow

| Example | Description |
|---------|-------------|
| **Countdown** | Count 10 → 0, writing each value to the LED register with delays. |
| **Fibonacci** | Print `fib(1)..fib(10)` (1 1 2 3 5 8 13 21 34 55) to UART. |
| **Nested Calls** | 3-level function call chain computing `((5 + 10) * 2) + 3 = 33`. Shows stack frame management with prologue/epilogue. |

### UART I/O

| Example | Description |
|---------|-------------|
| **UART Hello** | Send `"Hello\n"` character-by-character with TX busy-polling. |
| **Echo** | Interrupt-driven UART echo: lowercase → uppercase, `'!'` → halt. |

### Hardware I/O

| Example | Description |
|---------|-------------|
| **Blink LED** | Toggle LED D2 on/off in a loop with delay. |
| **Button Echo** | LED D2 follows button S2 state in real time. |

### Memory

| Example | Description |
|---------|-------------|
| **Memory Access** | Store/load from non-adjacent memory blocks (0x0100, 0x0200). Demonstrates the memory viewer's zero-row collapsing. |

## Source Files

Assembly sources are in `src/examples/assembler/`:

```
add.s  blink_led.s  button_echo.s  countdown.s  echo.s
fibonacci.s  memory_access.s  multiply.s  nested_calls.s
stack_variables.s  uart_hello.s
```

## See Also

- [Rust Pipeline Examples](rust-pipeline-demos.md) — Rust programs compiled through the Rust → MSP430 → COR24 pipeline
- [Live Web Emulator](https://sw-embed.github.io/cor24-rs/) — Browser-based emulator
