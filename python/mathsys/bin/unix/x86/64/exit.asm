;
;   HEAD
;

; HEAD -> GLOBALS
global exit

; HEAD -> MARK
section .text


;
;   FUNCTIONS
;

; FUNCTIONS -> EXIT
exit:
    mov rax, 60
    xor rdi, rdi
    syscall


;
;   BOTTOM
;

; BOTTOM -> LINUX NOTICE
section .note.GNU-no-entry