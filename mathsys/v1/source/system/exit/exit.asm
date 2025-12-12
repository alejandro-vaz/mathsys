;
;   HEAD
;

; HEAD -> GLOBALS
global systemExit

; HEAD -> MARK
section .text


;
;   SYSTEM
;

; SYSTEM -> EXIT
systemExit:;                                                                    systemExit(rdi)
    mov rax, 60
    syscall