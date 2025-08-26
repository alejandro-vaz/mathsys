#!/bin/bash
#
#   COMPILE
#

# COMPILE -> COMMAND
(
    cd python/mathsys/bin/unix/x86/64
    nasm -f elf64 all.asm -o all.o
)