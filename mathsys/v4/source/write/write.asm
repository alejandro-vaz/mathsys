;^
;^  WRITE
;^

;> WRITE -> FUNCTION
global _write
section .text
_write:
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