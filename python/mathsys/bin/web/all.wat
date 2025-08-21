;;
;;  HEAD
;;

;; HEAD -> MODULE
(module

;; HEAD -> IMPORTS
(import "env" "memory" (memory 0))
(import "sys" "call1" (func $call1 (param i32 i32)))
(import "sys" "call60" (func $call60 (param i32)))


;;
;;  SYSTEM
;;

;; SYSTEM -> EXIT
(func $systemExit (param $code i32);;                                           systemExit(code: i32)
    local.get $code
    call $call60
)(export "systemExit" (func $systemExit))

;; SYSTEM -> WRITE
(func $systemWrite (param $pointer i32);;                                       systemWrite(pointer: i32)
    (local $length i32)
    i32.const 0
    local.set $length
    block $break
        loop $scan
            local.get $pointer
            local.get $length
            i32.add
            i32.load8_u
            i32.eqz
            br_if $break
            local.get $length
            i32.const 1
            i32.add
            local.set $length
            br $scan
        end
    end
    local.get $pointer
    local.get $length
    call $call1
)(export "systemWrite" (func $systemWrite))


;;
;;  BOTTOM
;;

;; BOTTOM -> MARK
)
