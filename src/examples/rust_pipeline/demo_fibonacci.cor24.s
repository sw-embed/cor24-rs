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

; --- function: demo_fibonacci ---
demo_fibonacci:
    lc      r0, 10
    ; call fibonacci
    la      r2, .Lret_0
    push    r2
    la      r2, fibonacci
    jmp     (r2)
    .Lret_0:
    mov     r1, r0
    la      r0, 0x000100
    ; call mem_write
    la      r2, .Lret_1
    push    r2
    la      r2, mem_write
    jmp     (r2)
    .Lret_1:
.LBB1_1:
    bra     .LBB1_1
.Lfunc_end1:

; --- function: fibonacci ---
fibonacci:
    lw      r1, 15(fp)
    push    r1
    lw      r1, 18(fp)
    push    r1
    sw      r0, 15(fp)
    push    r0
    lc      r0, 1
    sw      r0, 18(fp)
    pop     r0
    push    r0
    lw      r0, 15(fp)
    push    r1
    lc      r1, 2
    clu     r0, r1
    pop     r1
    pop     r0
    brt     .LBB2_4
    push    r0
    lc      r0, 0
    sw      r0, 18(fp)
    pop     r0
.LBB2_2:
    lw      r0, 15(fp)
    add     r0, -1
    ; call fibonacci
    la      r2, .Lret_2
    push    r2
    la      r2, fibonacci
    jmp     (r2)
    .Lret_2:
    push    r1
    lw      r1, 18(fp)
    add     r1, r0
    sw      r1, 18(fp)
    pop     r1
    push    r0
    lw      r0, 15(fp)
    add     r0, -2
    sw      r0, 15(fp)
    pop     r0
    push    r0
    lw      r0, 15(fp)
    push    r1
    lc      r1, 2
    clu     r0, r1
    pop     r1
    pop     r0
    brf     .LBB2_2
    push    r0
    lw      r0, 18(fp)
    add     r0, 1
    sw      r0, 18(fp)
    pop     r0
.LBB2_4:
    lw      r0, 18(fp)
    pop     r1
    sw      r1, 18(fp)
    pop     r1
    sw      r1, 15(fp)
    pop     r2
    jmp     (r2)
.Lfunc_end2:

; --- function: mem_write ---
mem_write:
    sb      r1, 0(r0)
    pop     r2
    jmp     (r2)
.Lfunc_end3:

; --- function: start ---
start:
    ; call demo_fibonacci
    la      r2, .Lret_3
    push    r2
    la      r2, demo_fibonacci
    jmp     (r2)
    .Lret_3:
.Lfunc_end4:

