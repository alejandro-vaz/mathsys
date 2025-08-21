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

