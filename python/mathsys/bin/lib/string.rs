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