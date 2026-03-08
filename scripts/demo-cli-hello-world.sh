#!/bin/bash
# Demo: Hello World UART output
#
# Assembles and runs hello_world.s, capturing UART output to a shell variable.
# Shows the full pipeline: .s source → .lgo binary → emulator → UART output
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
DBG="$PROJECT_DIR/target/debug/cor24-dbg"
LGO="$PROJECT_DIR/tests/programs/hello_world.lgo"
SRC="$PROJECT_DIR/tests/programs/hello_world.s"

# Build the debugger
echo "=== Building cor24-dbg ==="
cargo build -p cor24-cli --manifest-path "$PROJECT_DIR/Cargo.toml" 2>&1

echo ""
echo "=== Source: hello_world.s ==="
cat "$SRC"

echo ""
echo "=== Running in emulator ==="
echo ""

# Run the debugger, capture full output
RAW_OUTPUT=$($DBG "$LGO" <<'CMDS'
run 1000
uart
quit
CMDS
)

echo "$RAW_OUTPUT"

echo ""
echo "=== Extracting UART output into shell variable ==="
echo ""

# Extract UART content: everything between "UART output buffer..." line and next "(cor24)" prompt
# The UART output appears on lines after the "UART output buffer (N chars):" header
UART_OUTPUT=$(echo "$RAW_OUTPUT" | awk '/UART output buffer/{found=1; next} found && /[(]cor24[)]/{found=0} found && NF{print}')

echo "UART_OUTPUT variable contains:"
echo "---"
echo "$UART_OUTPUT"
echo "---"
echo ""

# Verify the output
EXPECTED="Hello, World!"
if echo "$UART_OUTPUT" | grep -qF "$EXPECTED"; then
    echo "PASS: UART output contains '$EXPECTED'"
else
    echo "FAIL: Expected '$EXPECTED' in UART output"
    exit 1
fi

echo ""
echo "=== Using UART output in a pipeline ==="
echo ""

# Demonstrate piping UART output
CHAR_COUNT=$(echo -n "$UART_OUTPUT" | wc -c)
WORD_COUNT=$(echo "$UART_OUTPUT" | wc -w)
echo "Character count: $CHAR_COUNT"
echo "Word count: $WORD_COUNT"
echo "Uppercase: $(echo "$UART_OUTPUT" | tr '[:lower:]' '[:upper:]')"
echo "Hex dump:"
echo -n "$UART_OUTPUT" | xxd | head -3
