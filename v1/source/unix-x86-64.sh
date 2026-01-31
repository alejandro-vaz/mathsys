#!/bin/bash
#
#   COMPILE
#

# COMPILE -> COMMAND
(
    cd python/mathsys/dev/source
    cat > unix-x86-64.asm << 'EOF'
;
;   INCLUDE
;

; INCLUDE -> SYSTEM
%include "system/exit/exit.asm"
%include "system/write/write.asm"

; INCLUDE -> NOTICE
section .note.GNU-no-entry
EOF
    nasm -f elf64 unix-x86-64.asm -o unix-x86-64.o
    rm unix-x86-64.asm
)