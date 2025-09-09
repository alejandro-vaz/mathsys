#!/bin/bash
#
#   COMPILE
#

# COMPILE -> COMMAND
(
    cd python/mathsys/dev/source
    {
        cat << 'EOF'
        (module
        (import "env" "memory" (memory 0))
        (import "sys" "call1" (func $call1 (param i32 i32)))
        (import "sys" "call60" (func $call60 (param i32)))
EOF
        cat system/exit/exit.wat
        cat system/write/write.wat
        echo ")"
    } > web.wat
    wat2wasm web.wat -r -o web.wasm
    rm web.wat
    wasm-ld -flavor wasm -r web.wasm -o all.o
    rm web.wasm
)