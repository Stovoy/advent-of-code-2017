use utils;

pub fn run() {
    let input = utils::read_input(5);
    let jump_offsets: Vec<i32> = input.iter()
                                      .map(|n| n.parse::<i32>().unwrap())
                                      .collect();

    println!("{}", count_steps(jump_offsets.clone(), |v| v + 1));
    println!("{}", count_steps(jump_offsets.clone(), |v| if v >= 3 { v - 1} else { v + 1 }));
}

fn count_steps(mut jump_offsets: Vec<i32>, post_jump_modification_operator: fn(i32) -> i32) -> i32 {
    let mut instruction_pointer: i32 = 0;
    let mut steps = 0;

    loop {
        if instruction_pointer < 0 || instruction_pointer as usize >= jump_offsets.len() {
            break;
        }
        let jump_offset = jump_offsets[instruction_pointer as usize];
        jump_offsets[instruction_pointer as usize] = post_jump_modification_operator(jump_offset);
        instruction_pointer += jump_offset;
        steps += 1;
    }

    steps
}
