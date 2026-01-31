#!/bin/bash
#^
#^  COMPILE
#^

#> COMPILE -> COMMAND
(
    cd mathsys/dev/bin
    {
        cat << 'EOF'
        .include "exit/exit.s"
        .include "write/write.s"
EOF
    } > darwin.s
    clang -c -arch arm64 darwin.s -o darwin.o
    rm darwin.s
)
