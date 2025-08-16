;
;   HEAD
;

; HEAD -> GLOBALS
global numberAdd

; HEAD -> MARK
section .text


;
;   NUMBER
;

; NUMBER -> ADD
numberAdd:
    lea rax, [rdi + rsi]
    ret