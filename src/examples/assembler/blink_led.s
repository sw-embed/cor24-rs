; Blink LED: Toggle LED D2 on and off
; Hover over D2 to see duty cycle %
; LED D2 at address -65536 (write bit 0)
;
; Try changing the nop counts below to
; see the duty cycle change:
;   equal nops = ~50% duty cycle
;   more ON nops = higher duty cycle
;   more OFF nops = lower duty cycle

        la      r1,-65536

loop:
        lc      r0,1
        sb      r0,0(r1)    ; LED on
        ; On-time: add nops to increase duty cycle
        nop
        nop
        nop
        nop
        nop

        lc      r0,0
        sb      r0,0(r1)    ; LED off
        ; Off-time: add nops to decrease duty cycle
        nop
        nop
        nop
        nop
        nop

        bra     loop
