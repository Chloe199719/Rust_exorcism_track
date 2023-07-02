pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    if len == 0 {
        for _ in 0..digits.len() + 1 {
            result.push("".to_string());
        }
        return result;
    }
    if len > digits.len() {
        return result;
    }
    for i in 0..digits.len() - len + 1 {
        result.push(digits[i..i + len].to_string());
    }
    result
    // unimplemented!("What are the series of length {len} in string {digits:?}")
}
