use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = input.to_uppercase();
    let words  = puzzle.split_whitespace().filter(|&s| s != "+" && s != "==").map(|s| s.chars().collect()).collect::<Vec<String>>();
    let mut  unique_letters = HashSet::new();
    for word in &words {
        for letter in word.chars() {
            unique_letters.insert(letter);
        }
    }
    let mut leading_letters = HashSet::new();
    for word in &words {
        if let Some(letter) = word.chars().next() {
            leading_letters.insert(letter);
        }
    }

    if unique_letters.len() > 10  {
        return None;
    }
    let unique_letters: Vec<char> = unique_letters.into_iter().collect();
    let digits: Vec<u8> = (0..10).collect();
    
    unimplemented!("Solve the alphametic {input:?}")
}

