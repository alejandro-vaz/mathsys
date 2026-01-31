;;^
;;^ EXIT
;;^

;;> EXIT -> FUNCTION
(func $mathsys_exit (param $code i32)
    local.get $code
    call $call60
    unreachable
)(export "mathsys_exit" (func $mathsys_exit))