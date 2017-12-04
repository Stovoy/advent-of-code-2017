use utils;

pub fn run() {
    for line in utils::read_input(1) {
        let digits = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        println!("{}", sum_matching_digits(&digits, 1));
        println!("{}", sum_matching_digits(&digits, digits.len() / 2));
    }
}

fn sum_matching_digits(digits: &Vec<u32>, distance: usize) -> u32 {
    digits.iter()
        .enumerate()
        .filter(|&(i, _)| digits[i] == digits[(i + distance) % digits.len()])
        .map(|(_, digit)| digit)
        .sum()
}
