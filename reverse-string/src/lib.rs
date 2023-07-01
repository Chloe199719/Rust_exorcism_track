use std::io;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // let mut result = String::new();
    // io::stdin().read_line(&mut result).expect("Failed to read line");
   input.graphemes(true).rev().collect()
   
}
