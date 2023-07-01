use std::collections::HashSet;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

fn sort_string(s: &str) -> Vec<String> {
    let mut chars: Vec<String> = s
        .graphemes(true)
        .map(|g| g.nfkd().collect::<String>().to_lowercase())
        .collect();
    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word_sorted = sort_string(&word.to_lowercase());
    let word_lowercase = word.to_lowercase();

    for &possible_anagram in possible_anagrams {
        let possible_anagram_lowercase = possible_anagram.to_lowercase();

        // Skip if the word is equal to the possible anagram in any case
        if word == possible_anagram || word_lowercase == possible_anagram_lowercase {
            continue;
        }

        let possible_anagram_sorted = sort_string(&possible_anagram_lowercase);
        if word_sorted == possible_anagram_sorted {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}