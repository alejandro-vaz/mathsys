;;
;;  HEAD
;;

;; HEAD -> MODULE
(module

;; HEAD -> IMPORTS
(import "env" "memory" (memory 81))
(import "sys" "call60" (func $call60 (param i32)))


;;
;;  SYSTEM
;;

;; SYSTEM -> EXIT
(func $systemExit
    i32.const 0
    call $call60
)(export "systemExit" (func $systemExit)))