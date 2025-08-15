//
//  STRING
//

// STRING -> SPLIT
pub fn split(string: &str, splitter: &str) -> crate::Vec<crate::String> {
    let mut result = crate::Vec::new();
    let mut start = 0;
    while let Some(position) = string[start..].find(splitter) {
        let end = start + position;
        result.push(crate::String::from(&string[start..end]));
        start = end + splitter.len();
    }
    if start < string.len() {
        result.push(crate::String::from(&string[start..]));
    }
    return result;
}

// STRING -> JOIN
pub fn join(items: &[&str]) -> crate::String {
    let mut total_length = 0;
    for item in items {
        total_length += item.len();
    }
    let mut result = crate::String::with_capacity(total_length);
    for item in items {
        result.push_str(item);
    }
    return result;
}