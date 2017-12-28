use std::collections::HashSet;

use itertools::Itertools;

use utils;

pub fn run() {
    let mut total_valid_passphrases = 0;
    let mut total_valid_passphrases_sorted = 0;
    for line in utils::read_input(4) {
        let words: Vec<String> = line.split(' ').map(String::from).collect();
        if is_valid(&words) {
            total_valid_passphrases += 1;
        }
        if is_valid(&words.iter()
                          .map(|word| word.chars()
                                          .sorted()
                                          .iter()
                                          .collect())
                          .collect()) {
            total_valid_passphrases_sorted += 1;
        }
    }

    println!("{}", total_valid_passphrases);
    println!("{}", total_valid_passphrases_sorted);
}

fn is_valid(words: &Vec<String>) -> bool {
    let mut unique_words = HashSet::new();
    let mut valid = true;
    for word in words {
        if unique_words.contains(&word) {
            valid = false;
            break;
        }
        unique_words.insert(word);
    }

    valid
}
