use std::collections::HashSet;

use itertools::Itertools;

use utils;

pub fn run() {
    for line  in utils::read_input(6) {
        let mut memory_banks: Vec<i32> = line.split("\t").map(|n| n.parse::<i32>().unwrap()).collect();

        println!("{}", count_redistributions_until_duplicate_state(&mut memory_banks));
        println!("{}", count_redistributions_until_duplicate_state(&mut memory_banks));
    }
}

pub fn count_redistributions_until_duplicate_state(memory_banks: &mut Vec<i32>) -> i32 {
    let mut seen_memory_bank_states = HashSet::new();
    let mut redistributions = 0;
    loop {
        let memory_bank_state = memory_banks.iter().map(|n| n.to_string()).join("\t");
        if seen_memory_bank_states.contains(&memory_bank_state) {
            break;
        }
        seen_memory_bank_states.insert(memory_bank_state);

        let mut max = 0;
        let mut max_index = 0;
        for (i, blocks) in memory_banks.iter().enumerate() {
            if *blocks > max {
                max = *blocks;
                max_index = i;
            }
        }

        let mut blocks_to_redistribute = memory_banks[max_index];
        memory_banks[max_index] = 0;

        let mut index = max_index + 1;
        while blocks_to_redistribute > 0 {
            if index >= memory_banks.len() {
                index = 0;
            }

            blocks_to_redistribute -= 1;
            memory_banks[index] += 1;
            index += 1;
        }

        redistributions += 1;
    }

    redistributions
}
