;; SYSTEM -> EXIT
(func $systemExit (param $code i32);;                                           systemExit(code: i32)
    local.get $code
    call $call60
)(export "systemExit" (func $systemExit))

