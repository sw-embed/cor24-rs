; Assert: Show how to validate results
;
; Pattern: compare actual vs expected,
; branch to assert_fail if wrong.
; On failure the emulator halts at
; assert_fail — inspect registers to debug.
;
; This example has a DELIBERATE BUG in
; test 3 to show what failure looks like.
; Fix the expected value to make it pass!

        ; --- Test 1: addition ---
        lc      r0,17
        lc      r1,25
        add     r0,r1           ; r0 = 42
        ; assert r0 == 42
        lc      r1,42
        ceq     r0,r1
        brf     assert_fail     ; PASS

        ; --- Test 2: native multiply ---
        lc      r0,6
        lc      r1,7
        mul     r0,r1           ; r0 = 42
        ; assert r0 == 42
        lc      r1,42
        ceq     r0,r1
        brf     assert_fail     ; PASS

        ; --- Test 3: subtract (DELIBERATE BUG) ---
        lc      r0,100
        lc      r1,30
        sub     r0,r1           ; r0 = 70
        ; assert r0 == 70
        ; BUG: wrong expected value!
        ; Change 99 to 70 to fix this test.
        lc      r1,99
        ceq     r0,r1
        brf     assert_fail     ; FAILS here

        ; --- All tests passed ---
all_pass:
        bra     all_pass

        ; --- Assertion failed ---
        ; Halts here. Step back and check
        ; registers to see actual vs expected.
assert_fail:
        bra     assert_fail
