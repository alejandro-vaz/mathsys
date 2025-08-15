;;
;;  HEAD
;;

;; HEAD -> MODULE
(module

;; HEAD -> IMPORTS
(import "env" "memory" (memory 81))
(import "sys" "call1" (func $call1 (param i32 i32)))


;;
;;  FUNCTIONS
;;

;; FUNCTIONS -> WRITE
(func $write (param $ptr i32)
    (local $len i32)
    (local $cur i32)
    local.get $ptr
    local.set $cur
    i32.const 0
    local.set $len
    block $break
        loop $scan
            local.get $cur
            local.get $len
            i32.add
            i32.load8_u
            i32.eqz
            br_if $break
            local.get $len
            i32.const 1
            i32.add
            local.set $len
            br $scan
        end
    end
    local.get $ptr
    local.get $len
    call $call1
)


;;
;;  EXPORTS
;;

;; EXPORTS -> WRITE
(export "write" (func $write))


;;
;;  BOTTOM
;;

;; BOTTOM -> MARK
)