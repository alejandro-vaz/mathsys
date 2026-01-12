;^
;^  WRITE
;^

;> WRITE -> FUNCTION
.global _mathsys_write
_mathsys_write:
    mov x1, x0
    mov x2, 0
    .find:
        ldrb w3, [x1, x2]
        cbz w3, .found
        add x2, x2, 1
        b .find
    .found:
        mov x0, 1
        ldr x16, =0x2000004
        svc 0
        ret