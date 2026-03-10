; COR24 Interrupt Example
; Main loop increments a counter (0-9), storing it in memory.
; When a character is sent via UART, the interrupt fires and
; the ISR prints the current counter value as an ASCII digit.
;
; Usage: Assemble & Run, then type characters in the UART input.
; Each character you send triggers an interrupt that prints the
; current counter value (0-9) to UART output.

; --- Setup interrupt vector ---
        la      r0, isr         ; address of ISR
        mov     iv, r0          ; set interrupt vector (r6)

; --- Enable UART RX interrupt ---
        la      r1, 0xFF0010    ; interrupt enable register
        lc      r0, 1
        sb      r0, 0(r1)       ; enable bit 0

; --- Main loop: count 0-9 forever ---
; Counter stored at address 0x0100 so ISR can read it
        la      r1, 0x0100      ; counter memory location
        lc      r0, 0           ; counter = 0
loop:
        sb      r0, 0(r1)       ; store counter to memory
        lc      r2, 1
        add     r0, r2          ; counter++
        lc      r2, 10
        ceq     r0, r2          ; counter == 10?
        brf     loop            ; no -> keep counting
        lc      r0, 0           ; wrap to 0
        bra     loop

; --- Interrupt Service Routine ---
; Saves registers, reads UART RX (acknowledges interrupt),
; prints counter as ASCII digit, restores and returns.
isr:
        push    r0
        push    r1
        push    r2
        mov     r2, c           ; save condition flag
        push    r2

        ; Read UART RX byte to acknowledge interrupt
        la      r1, 0xFF0100    ; UART data register
        lb      r2, 0(r1)      ; read and discard RX byte

        ; Read current counter from memory
        la      r1, 0x0100      ; counter memory location
        lb      r2, 0(r1)      ; r2 = counter value

        ; Convert to ASCII digit: '0' + counter
        lc      r0, 48          ; ASCII '0'
        add     r0, r2          ; r0 = ASCII digit

        ; Write to UART TX
        la      r1, 0xFF0100
        sb      r0, 0(r1)      ; transmit digit

        ; Restore registers
        pop     r2
        clu     z, r2           ; restore condition flag
        pop     r2
        pop     r1
        pop     r0
        jmp     (ir)            ; return from interrupt (clears intis)
