;^
;^  EXIT
;^

;> EXIT -> FUNCTION
global mathsys_exit
section .text
mathsys_exit:
    mov rax, 60
    syscall
    ud2