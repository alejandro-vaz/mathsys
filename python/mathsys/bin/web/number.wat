;;
;;  HEAD
;;

;; HEAD -> MODULE
(module

;; HEAD -> IMPORTS
(import "env" "memory" (memory 81))


;;
;;  FUNCTIONS
;;

;; FUNCTIONS -> NUMBERADD
(func $numberAdd (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
)


;;
;;  EXPORTS
;;

;; EXPORTS -> NUMBERADD
(export "numberAdd" (func $numberAdd))


;;
;;  BOTTOM
;;

;; BOTTOM -> MARK
)