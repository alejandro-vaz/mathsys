;
;   HEAD
;

; HEAD -> GLOBALS
global write

; HEAD -> MARK
section .text


;
;   FUNCTIONS
;

; FUNCTIONS -> WRITE
write:
    mov rsi, rdi
    xor rcx, rcx
    find:
        cmp byte [rsi + rcx], 0
        je found
        inc rcx
        jmp find
    found:
        mov rax, 1
        mov rdi, 1
        mov rdx, rcx
        syscall
        ret


;
;   BOTTOM
;

; BOTTOM -> LINUX NOTICE
section .note.GNU-no-entry