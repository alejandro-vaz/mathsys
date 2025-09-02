;
;   HEAD
;

; HEAD -> GLOBALS
global systemWrite

; HEAD -> MARK
section .text


;
;   SYSTEM
;

; SYSTEM -> WRITE
systemWrite:;                                                                   systemWrite(rdi)
    mov rsi, rdi
    xor rcx, rcx
    .find:
        cmp byte [rsi + rcx], 0
        je .found
        inc rcx
        jmp .find
    .found:
        mov rax, 1
        mov rdi, 1
        mov rdx, rcx
        syscall
        ret