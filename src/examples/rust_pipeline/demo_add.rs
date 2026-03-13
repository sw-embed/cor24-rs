//! Demo: Add Two Numbers
//! Computes 100 + 200 + 42 = 342 and stores result at address 0x0100.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s → assembler → emulator

#![no_std]

const UART_DATA: u16 = 0xFF01;
/// Result stored here — visible in memory viewer at halt
const RESULT_ADDR: u16 = 0x0100;

#[inline(never)]
#[no_mangle]
pub unsafe fn mmio_write(addr: u16, val: u16) {
    core::ptr::write_volatile(addr as *mut u8, val as u8);
}

#[inline(never)]
#[no_mangle]
pub unsafe fn uart_putc(ch: u16) {
    mmio_write(UART_DATA, ch);
}

#[inline(never)]
#[no_mangle]
pub fn demo_add() -> u16 {
    let a: u16 = 100;
    let b: u16 = 200;
    let c: u16 = 42;
    a + b + c
}

/// Entry point — compute sum, store to memory, halt
#[inline(never)]
#[no_mangle]
pub unsafe fn start() -> ! {
    let result = demo_add();
    core::ptr::write_volatile(RESULT_ADDR as *mut u16, result);
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        uart_putc(b'P' as u16);
        uart_putc(b'A' as u16);
        uart_putc(b'N' as u16);
        uart_putc(b'I' as u16);
        uart_putc(b'C' as u16);
        uart_putc(b'\n' as u16);
    }
    loop {}
}
