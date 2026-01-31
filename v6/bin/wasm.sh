#!/bin/bash
#^
#^  COMPILE
#^

#> COMPILE -> COMMAND
(
    cd mathsys/dev/bin
    {
        cat << 'EOF'
        (module
        (import "env" "memory" (memory 0))
        (import "sys" "call1" (func $call1 (param i32 i32)))
        (import "sys" "call60" (func $call60 (param i32)))
EOF
        cat exit/exit.wat
        cat write/write.wat
        echo ")"
    } > wasm.wat
    wat2wasm wasm.wat -r -o wasm.wasm
    rm wasm.wat
    wasm-ld -flavor wasm -r wasm.wasm -o wasm.o
    rm wasm.wasm
)