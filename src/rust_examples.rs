//! Rust pipeline example data — loaded from files via include_str!()

use components::RustExample;

fn example(name: &str, description: &str, rust: &str, msp430: &str, cor24: &str) -> RustExample {
    RustExample {
        name: name.to_string(),
        description: description.to_string(),
        rust_source: rust.to_string(),
        msp430_asm: msp430.to_string(),
        cor24_assembly: cor24.to_string(),
    }
}

pub fn get_rust_examples() -> Vec<RustExample> {
    vec![
        example(
            "Add Two Numbers",
            "Compute 100 + 200 + 42 = 342, store to memory",
            include_str!("examples/rust_pipeline/demo_add.rs"),
            include_str!("examples/rust_pipeline/demo_add.msp430.s"),
            include_str!("examples/rust_pipeline/demo_add.cor24.s"),
        ),
        example(
            "Blink LED",
            "Toggle LED with delay loop",
            include_str!("examples/rust_pipeline/demo_blinky.rs"),
            include_str!("examples/rust_pipeline/demo_blinky.msp430.s"),
            include_str!("examples/rust_pipeline/demo_blinky.cor24.s"),
        ),
        example(
            "Button Echo",
            "LED follows button S2 (pressed = on)",
            include_str!("examples/rust_pipeline/demo_button_echo.rs"),
            include_str!("examples/rust_pipeline/demo_button_echo.msp430.s"),
            include_str!("examples/rust_pipeline/demo_button_echo.cor24.s"),
        ),
        example(
            "Countdown",
            "Count 10→0, store each value to memory",
            include_str!("examples/rust_pipeline/demo_countdown.rs"),
            include_str!("examples/rust_pipeline/demo_countdown.msp430.s"),
            include_str!("examples/rust_pipeline/demo_countdown.cor24.s"),
        ),
        example(
            "Drop (RAII)",
            "Automatic destructor on scope exit — no allocator needed",
            include_str!("examples/rust_pipeline/demo_drop.rs"),
            include_str!("examples/rust_pipeline/demo_drop.msp430.s"),
            include_str!("examples/rust_pipeline/demo_drop.cor24.s"),
        ),
        example(
            "Echo (Interrupts)",
            "UART echo with Rust logic + asm!() interrupt plumbing",
            include_str!("examples/rust_pipeline/demo_echo.rs"),
            include_str!("examples/rust_pipeline/demo_echo.msp430.s"),
            include_str!("examples/rust_pipeline/demo_echo.cor24.s"),
        ),
        example(
            "Fibonacci (iterative)",
            "Compute fib(10) = 89 iteratively, store to memory",
            include_str!("examples/rust_pipeline/demo_fibonacci_iter.rs"),
            include_str!("examples/rust_pipeline/demo_fibonacci_iter.msp430.s"),
            include_str!("examples/rust_pipeline/demo_fibonacci_iter.cor24.s"),
        ),
        example(
            "Fibonacci (recursive)",
            "Compute fib(10) = 89 recursively, store to memory",
            include_str!("examples/rust_pipeline/demo_fibonacci.rs"),
            include_str!("examples/rust_pipeline/demo_fibonacci.msp430.s"),
            include_str!("examples/rust_pipeline/demo_fibonacci.cor24.s"),
        ),
        example(
            "Nested Calls",
            "3-level call chain, result stored to memory",
            include_str!("examples/rust_pipeline/demo_nested.rs"),
            include_str!("examples/rust_pipeline/demo_nested.msp430.s"),
            include_str!("examples/rust_pipeline/demo_nested.cor24.s"),
        ),
        example(
            "Panic Handler",
            "Turns on LED, prints PANIC to UART, halts",
            include_str!("examples/rust_pipeline/demo_panic.rs"),
            include_str!("examples/rust_pipeline/demo_panic.msp430.s"),
            include_str!("examples/rust_pipeline/demo_panic.cor24.s"),
        ),
        example(
            "Stack Variables",
            "Accumulate values across many variables, result to memory",
            include_str!("examples/rust_pipeline/demo_stack_vars.rs"),
            include_str!("examples/rust_pipeline/demo_stack_vars.msp430.s"),
            include_str!("examples/rust_pipeline/demo_stack_vars.cor24.s"),
        ),
        example(
            "UART Hello",
            "Send Hello to UART output",
            include_str!("examples/rust_pipeline/demo_uart_hello.rs"),
            include_str!("examples/rust_pipeline/demo_uart_hello.msp430.s"),
            include_str!("examples/rust_pipeline/demo_uart_hello.cor24.s"),
        ),
    ]
}
