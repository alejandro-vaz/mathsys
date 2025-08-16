//
//  FORMATTING
//

// FORMATTING -> FUNCTION
unsafe fn print(string: &str, append: &[u8]) -> () {
    let mut bytes = crate::Vec::new();
    bytes.extend_from_slice(append);
    bytes.extend_from_slice(string.as_bytes());
    bytes.extend_from_slice(&[0x1B, 0x5B, 0x30, 0x6D]);
    bytes.push(0x0A);
    bytes.push(0x00);
    crate::stack::system::write(bytes.as_ptr());
}


//
//  BB CALLS
//

// BB CALLS -> LOGIN
pub unsafe fn login() -> () {
    print(
        &crate::string::join(&[
            "LOGIN: ", 
            "Running Mathsys v", 
            crate::SETTINGS.version,
            ". Consuming ",
            &crate::alloc::format!("{}", crate::SETTINGS.ir.len()),
            " tokens."
        ]), 
        &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x32, 0x3B, 0x34, 0x39, 0x6D]
    );
}

// BB CALLS -> CRASH
pub unsafe fn crash() -> ! {
    print(
        &crate::string::join(&[
            "\n",
            "CRASH: ",
            "Process finished."
        ]),
        &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x31, 0x3B, 0x34, 0x39, 0x6D]
    );
    crate::stack::system::exit();
}


//
//  B CALLS
//

// B CALLS -> SPACE
pub unsafe fn space(message: &str) -> () {
    if crate::SETTINGS.bcalls {
        print(
            &crate::string::join(&[
                "\n",
                "SPACE: ", 
                message
            ]), 
            &[0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}

// B CALLS -> ISSUE
pub unsafe fn issue(message: &str) -> () {
    if crate::SETTINGS.bcalls {
        print(
            &crate::string::join(&[
                "\n",
                "ISSUE: ", 
                message
            ]), 
            &[0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x31, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}


//
//  N CALLS
//

// N CALLS -> DEBUG
pub unsafe fn debug(message: &str) -> () {
    if crate::SETTINGS.ncalls {
        print(
            &crate::string::join(&[
                "    ",
                "DEBUG: ",
                message
            ]),
            &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}

// N CALLS -> ALERT
pub unsafe fn alert(message: &str) -> () {
    if crate::SETTINGS.ncalls {
        print(
            &crate::string::join(&[
                "    ",
                "ALERT: ",
                message
            ]),
            &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}

// N CALLS -> TRACE
pub unsafe fn trace(message: &str) -> () {
    if crate::SETTINGS.ncalls {
        print(
            &crate::string::join(&[
                "    ",
                "TRACE: ",
                message
            ]),
            &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}