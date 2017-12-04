use std::iter::FromIterator;

use utils;

pub fn run() {
    for line in utils::read_input(1) {
        let digits = Vec::from_iter(line.chars().map(|c| c.to_digit(10).unwrap()));
        println!("{0}", sum_matching_digits(&digits, 1));
        println!("{0}", sum_matching_digits(&digits, digits.len() / 2));
    }
}

fn sum_matching_digits(digits: &Vec<u32>, distance: usize) -> u32 {
    let mut sum = 0;

    for i in 0..digits.len() {
        if is_matched(digits, i, distance) {
            sum += &digits[i];
        }
    }

    return sum;
}

fn is_matched(digits: &Vec<u32>, index: usize, distance: usize) -> bool {
    let other_index = (index + distance) % digits.len();
    return digits[index] == digits[other_index];
}
