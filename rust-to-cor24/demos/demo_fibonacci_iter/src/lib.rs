//! Demo: Fibonacci (iterative)
//! Computes fib(10) = 89 using a simple loop with only 3 variables.
//! Iterative style avoids deep recursion and needs minimal registers.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s → assembler → emulator

#![no_std]

const RESULT_ADDR: u16 = 0x0100;

#[inline(never)]
#[no_mangle]
pub unsafe fn mem_write(addr: u16, val: u8) {
    core::ptr::write_volatile(addr as *mut u8, val);
}

/// Iterative fibonacci: fib(0)=1, fib(1)=1, fib(n)=fib(n-1)+fib(n-2)
/// Uses only 3 live variables (a, b, temp) — fits in COR24's 3 GP registers.
#[inline(never)]
#[no_mangle]
pub fn fibonacci_iter(n: u16) -> u16 {
    let mut a: u16 = 1;
    let mut b: u16 = 1;
    let mut i: u16 = 0;
    while i < n {
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    }
    a
}

#[inline(never)]
#[no_mangle]
pub unsafe fn demo_fibonacci_iter() {
    let result = fibonacci_iter(10);  // Should be 89
    mem_write(RESULT_ADDR, result as u8);
    loop {}
}

/// Entry point
#[inline(never)]
#[no_mangle]
pub unsafe fn start() -> ! {
    demo_fibonacci_iter();
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
