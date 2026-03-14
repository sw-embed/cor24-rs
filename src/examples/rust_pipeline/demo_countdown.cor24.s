; COR24 Assembly - Generated from MSP430 via msp430-to-cor24
; Pipeline: Rust -> rustc (msp430-none-elf) -> MSP430 ASM -> COR24 ASM

; Reset vector -> start
    mov     fp, sp
    la      r0, start
    jmp     (r0)

; --- function: _RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind ---
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB0_1:
    bra     .LBB0_1
.Lfunc_end0:

; --- function: delay ---
delay:
    sub     sp, 3
    ceq     r0, z
    brt     .LBB1_3
    add     r0, -1
.LBB1_2:
    mov     r2, sp
    sw      r0, 0(r2)
    add     r0, -1
    push    r1
    lc      r1, -1
    ceq     r0, r1
    pop     r1
    brf     .LBB1_2
.LBB1_3:
    add     sp, 3
    pop     r2
    jmp     (r2)
.Lfunc_end1:

; --- function: demo_countdown ---
demo_countdown:
    lw      r1, 18(fp)
    push    r1
    push    r0
    lc      r0, 10
    sw      r0, 18(fp)
    pop     r0
.LBB2_1:
    la      r0, 0x000100
    lw      r1, 18(fp)
    ; call mem_write
    la      r2, .Lret_0
    push    r2
    la      r2, mem_write
    jmp     (r2)
    .Lret_0:
    la      r0, 0x0003E8
    ; call delay
    la      r2, .Lret_1
    push    r2
    la      r2, delay
    jmp     (r2)
    .Lret_1:
    push    r0
    lw      r0, 18(fp)
    add     r0, -1
    sw      r0, 18(fp)
    pop     r0
    push    r0
    lw      r0, 18(fp)
    ceq     r0, z
    pop     r0
    brf     .LBB2_1
    la      r0, 0x000100
    lc      r1, 0
    ; call mem_write
    la      r2, .Lret_2
    push    r2
    la      r2, mem_write
    jmp     (r2)
    .Lret_2:
.LBB2_3:
    bra     .LBB2_3
.Lfunc_end2:

; --- function: mem_write ---
mem_write:
    sb      r1, 0(r0)
    pop     r2
    jmp     (r2)
.Lfunc_end3:

; --- function: start ---
start:
    ; call demo_countdown
    la      r2, .Lret_3
    push    r2
    la      r2, demo_countdown
    jmp     (r2)
    .Lret_3:
.Lfunc_end4:

