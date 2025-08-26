//
//  FORMATTING
//

// FORMATTING -> FUNCTION
unsafe fn print(string: &str, append: &[u8]) -> () {
    let mut bytes = crate::Vec::new();
    let signature = signature();
    bytes.extend_from_slice(append);
    let available_space = (crate::SETTINGS.width as usize) - signature.len();
    let mut padded_string = crate::String::with_capacity(crate::SETTINGS.width as usize);
    let string_chars: crate::Vec<char> = string.chars().collect();
    if string_chars.len() >= available_space {
        for i in 0..available_space {
            padded_string.push(string_chars[i]);
        }
    } else {
        padded_string.push_str(string);
        for _ in string_chars.len()..available_space {
            padded_string.push(' ');
        }
    }
    padded_string.push_str(&signature);
    bytes.extend_from_slice(padded_string.as_bytes());
    bytes.extend_from_slice(&[0x1B, 0x5B, 0x30, 0x6D]);
    bytes.push(0x0A);
    bytes.push(0x00);
    crate::stack::system::write(bytes.as_ptr());
}

// FORMATTING -> MEMORY SIGNATURE
unsafe fn signature() -> crate::String {
    let number = crate::ALLOCATOR.next.load(crate::Ordering::Relaxed) - crate::ALLOCATOR.start;
    return crate::format!(
        "    {}",
        if number < 1000 {
            crate::format!("{:>6}", number)
        } else if number < 1000*1000 {
            crate::format!(
                "{:>3}.{}k",
                number / 1000,
                number % 1000 / 100
            )
        } else {
            crate::format!(
                "{:>3}.{}m",
                number / (1000*1000),
                number % (1000*1000) / (100*1000)
            )
        }
    );
}


//
//  BB CALLS
//

// BB CALLS -> LOGIN
pub unsafe fn login() -> () {
    print(
        &crate::format!(
            "LOGIN: Running Mathsys v{}.{}.{}, consuming {} tokens.",
            crate::SETTINGS.version[0],
            crate::SETTINGS.version[1],
            crate::SETTINGS.version[2],
            &crate::SETTINGS.ir.len()
        ), 
        &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x32, 0x3B, 0x34, 0x39, 0x6D]
    );
}

// BB CALLS -> CRASH
pub unsafe fn crash(code: u8) -> ! {
    print(
        &crate::format!(
            "CRASH: {}.",
            match code {
                0 => "Run finished successfully",
                1 => "Out of memory",
                255 => panic!(),
                _ => "Unknown reason"
            }
        ),
        &[0x0A, 0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x31, 0x3B, 0x34, 0x39, 0x6D]
    );
    crate::stack::system::exit(code);
}


//
//  B CALLS
//

// B CALLS -> SPACE
pub unsafe fn space(message: &str) -> () {
    if crate::SETTINGS.bcalls {
        print(
            &crate::format!(
                "SPACE: {}.",
                message
            ),
            &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}

// B CALLS -> ISSUE
pub unsafe fn issue(message: &str) -> () {
    if crate::SETTINGS.bcalls {
        print(
            &crate::format!(
                "ISSUE: {}.",
                message
            ),
            &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x31, 0x3B, 0x34, 0x39, 0x6D]
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
            &crate::format!(
                "    DEBUG: {}.",
                message
            ),
            &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}

// N CALLS -> ALERT
pub unsafe fn alert(message: &str) -> () {
    if crate::SETTINGS.ncalls {
        print(
            &crate::format!(
                "    ALERT: {}.",
                message
            ),
            &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}

// N CALLS -> TRACE
pub unsafe fn trace(message: &str) -> () {
    if crate::SETTINGS.ncalls {
        print(
            &crate::format!(
                "    TRACE: {}.",
                message
            ),
            &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D]
        )
    }
}