;
;   HEAD
;

; HEAD -> GLOBALS
global numberAdd

; HEAD -> MARK
section .text


;
;   FUNCTIONS
;

; FUNCTIONS -> NUMBERADD
numberAdd:
    lea rax, [rdi + rsi]
    ret


;
;   BOTTOM
;

; BOTTOM -> LINUX NOTICE
section .note.GNU-no-entry