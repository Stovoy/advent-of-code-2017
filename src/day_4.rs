use std::collections::HashSet;

use itertools::Itertools;

use utils;

pub fn run() {
    let mut total_valid_passphrases = 0;
    for line in utils::read_input(4) {
        let words: Vec<String> = line.split(' ').map(String::from).collect();
        let mut unique_words = HashSet::new();
        let mut valid = true;
        for word in words {
            let sorted_word = word.chars().sorted();

            if unique_words.contains(&sorted_word) {
                valid = false;
                break;
            }
            unique_words.insert(sorted_word);
        }
        if valid {
            total_valid_passphrases += 1;
        }
    }
    println!("{}", total_valid_passphrases);
}
