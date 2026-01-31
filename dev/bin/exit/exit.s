;^
;^  EXIT
;^

;> EXIT -> FUNCTION
.global _mathsys_exit
_mathsys_exit:
    ldr x16, =0x2000001
    svc 0
    brk #0