;;
;;  HEAD
;;

;; HEAD -> MODULE
(module

;; HEAD -> IMPORTS
(import "env" "memory" (memory 81))
(import "sys" "call60" (func $call60 (param i32)))


;;
;;  FUNCTIONS
;;

;; FUNCTIONS -> EXIT
(func $exit
    i32.const 0
    call $call60
)


;;
;;  EXPORTS
;;

;; EXPORTS -> EXIT
(export "exit" (func $exit))


;;
;;  BOTTOM
;;

;; BOTTOM -> MARK
)