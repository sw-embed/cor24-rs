; COR24 Assembly - Generated from MSP430 via msp430-to-cor24
; Pipeline: Rust -> rustc (msp430-none-elf) -> MSP430 ASM -> COR24 ASM

; Reset vector -> start
    mov     fp, sp
    la      r0, start
    jmp     (r0)

; --- function: demo_add ---
demo_add:
    la      r0, 0x000156
    jmp     (r1)
.Lfunc_end0:

; --- function: start ---
start:
    ; call demo_add
    push    r1
    la      r2, demo_add
    jal     r1, (r2)
    pop     r1
    ; TODO: mov (unsupported mov operand combination: Register(12) -> Absolute(()))
.LBB1_1:
    bra     .LBB1_1
.Lfunc_end1:

; --- function: _RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind ---
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB2_1:
    bra     .LBB2_1
.Lfunc_end2:


