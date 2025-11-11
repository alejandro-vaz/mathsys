;^
;^  EXIT
;^

;> EXIT -> FUNCTION
global _exit
section .text
_exit:
    mov rax, 60
    syscall
    ud2