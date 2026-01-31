;;^
;;^ WRITE
;;^

;;> WRITE -> FUNCTION
(func $_write (param $pointer i32)
    (local $current i32)
    local.get $pointer
    local.set $current
    block $break
        loop $scan
            local.get $current
            i32.load8_u
            i32.eqz
            br_if $break
            local.get $current
            i32.const 1
            i32.add
            local.set $current
            br $scan
        end
    end
    local.get $current
    local.get $pointer
    i32.sub
    local.get $pointer
    call $call1
)(export "_write" (func $_write))