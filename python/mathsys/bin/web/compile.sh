#!/bin/bash
#
#   COMPILE
#

# COMPILE -> COMMAND
(
    cd python/mathsys/bin/web
    cat > all.wat << 'EOF'
;;
;;  HEAD
;;

;; HEAD -> MODULE
(module

;; HEAD -> IMPORTS
(import "env" "memory" (memory 0))
(import "sys" "call1" (func $call1 (param i32 i32)))
(import "sys" "call60" (func $call60 (param i32)))

EOF
    cat >> all.wat << 'EOF'

;;
;;  SYSTEM
;;

EOF
    cat system/exit.wat >> all.wat
    cat system/write.wat >> all.wat
    cat >> all.wat << 'EOF'

;;
;;  BOTTOM
;;

;; BOTTOM -> MARK
)
EOF
    wat2wasm all.wat -r -o all.wasm
    wasm-ld -flavor wasm -r all.wasm -o all.o
    rm all.wasm
)