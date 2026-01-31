#!/bin/bash
#
#   COMPILE
#

# COMPILE -> COMMAND
(
    cd python/mathsys/dev/source
    {
        cat << 'EOF'
        %include "system/exit/exit.asm"
        %include "system/write/write.asm"
        section .note.GNU-no-entry
EOF
    } > unix-x86-64.asm
    nasm -f elf64 unix-x86-64.asm -o unix-x86-64.o
    rm unix-x86-64.asm
)