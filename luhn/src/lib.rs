/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    
    if digits.len()<=1{
        return false;
    }
    digits.iter()
    .rev()
    .enumerate()
    .try_fold(0, |acc, (i, c)| {
        c.to_digit(10)
        .map(|d| if i % 2 == 0 { d } else { d * 2 })
        .map(|d| if d > 9 { d - 9 } else { d })
        .map(|d| acc + d)
    })
    .map_or(false, |sum| sum % 10 == 0)
  
}
