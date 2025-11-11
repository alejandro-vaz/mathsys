;;^
;;^ EXIT
;;^

;;> EXIT -> FUNCTION
(func $_exit (param $code i32)
    local.get $code
    call $call60
    unreachable
)(export "_exit" (func $_exit))