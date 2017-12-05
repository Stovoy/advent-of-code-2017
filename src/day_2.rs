use utils;

pub fn run() {
    let input = utils::read_input(2);
    let number_lines: Vec<Vec<u32>> =
        input.iter()
             .map(|line| line.split('\t')
                             .map(|n| n.parse::<u32>().unwrap())
                             .collect())
             .collect();

    println!("{}", get_checksum(
        &number_lines, &min_max_difference_operator));
    println!("{}", get_checksum(
        &number_lines, &evenly_divisible_division_operator));
}

fn get_checksum(number_lines: &Vec<Vec<u32>>, line_value_operator: &Fn(&Vec<u32>) -> u32) -> u32 {
    let mut checksum = 0;
    for number_line in number_lines {
        checksum += line_value_operator(number_line);
    }
    checksum
}

fn min_max_difference_operator(numbers: &Vec<u32>) -> u32 {
    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();
    max - min
}

fn evenly_divisible_division_operator(numbers: &Vec<u32>) -> u32 {
    for (i, n1) in numbers.iter().enumerate() {
        for n2 in numbers[i + 1..].iter() {
            if n1 % n2 == 0 {
                return n1 / n2;
            } else if n2 % n1 == 0 {
                return n2 / n1;
            }
        }
    }

    0
}
