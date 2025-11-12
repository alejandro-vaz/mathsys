//^
//^ FORMATTING
//^

//> FORMATTING -> SCIENTIFIC
pub fn scientific(number: usize) -> crate::String {
    let k = 1_000;
    let m = 1_000_000;
    return if number < k {crate::format!(
        "{:>6}", 
        number
    )} else if number < m {crate::format!(
        "{:>3}.{}K",
        number / k,
        number % k / (k / 10)
    )} else {crate::format!(
        "{:>3}.{}M",
        number / m,
        number % m / (m / 10)
    )}
}