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

; --- function: _RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop ---
_RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop:
    lw      r0, 0(r0)
    lc      r1, 0
    ; tail call mem_write
    la      r2, mem_write
    jmp     (r2)
.Lfunc_end1:

; --- function: guard_new ---
guard_new:
    lw      r1, 18(fp)
    push    r1
    sw      r0, 18(fp)
    lc      r1, 1
    ; call mem_write
    la      r2, .Lret_0
    push    r2
    la      r2, mem_write
    jmp     (r2)
    .Lret_0:
    lw      r0, 18(fp)
    pop     r1
    sw      r1, 18(fp)
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
    sub     sp, 3
    la      r0, 0x000100
    ; call guard_new
    la      r2, .Lret_1
    push    r2
    la      r2, guard_new
    jmp     (r2)
    .Lret_1:
    la      r0, 0x000100
    mov     r2, sp
    sw      r0, 0(r2)
    mov     r0, sp
    ; call _RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop
    la      r2, .Lret_2
    push    r2
    la      r2, _RNvXs_Csdm5oPmm48S1_9demo_dropNtB4_5GuardNtNtNtCshbXD54rZpVC_4core3ops4drop4Drop4drop
    jmp     (r2)
    .Lret_2:
    la      r0, 0x000100
    la      r1, 0x0000FF
    ; call mem_write
    la      r2, .Lret_3
    push    r2
    la      r2, mem_write
    jmp     (r2)
    .Lret_3:
.LBB4_1:
    bra     .LBB4_1
.Lfunc_end4:

