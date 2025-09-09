; SYSTEM -> EXIT
global systemExit
section .text
systemExit:;                                                                    systemExit(rdi)
    mov rax, 60
    syscall