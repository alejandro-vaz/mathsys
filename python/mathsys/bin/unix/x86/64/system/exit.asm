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
systemExit:
    mov rax, 60
    xor rdi, rdi
    syscall