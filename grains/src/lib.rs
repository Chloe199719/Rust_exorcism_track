use std::collections::HashMap;
pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    let mut hashMap = HashMap::new();
    for i in 1..65 {
        hashMap.insert(i, 2u64.pow(i-1));
    }
    *hashMap.get(&s).unwrap()
}

pub fn total() -> u64 {
    let mut hashMap = HashMap::new();
    for i in 1..65 {
        hashMap.insert(i, 2u64.pow(i-1));
    }
    let mut sum: u64 = 0;
    for i in 1..65 {
        sum += *hashMap.get(&i).unwrap();
    }
    sum
}
