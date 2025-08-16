;;
;;  HEAD
;;

;; HEAD -> MODULE
(module

;; HEAD -> IMPORTS
(import "env" "memory" (memory 81))


;;
;;  NUMBER
;;

;; NUMBER -> ADD
(func $numberAdd (param $first i32) (param $second i32) (result i32)
    local.get $first
    local.get $second
    i32.add
)(export "numberAdd" (func $numberAdd)))