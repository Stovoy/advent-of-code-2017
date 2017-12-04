use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;


pub fn read_input(day: u8) -> Vec<String> {
    let filename = format!("inputs/day_{0}.txt", day);
    let mut file = File::open(&filename).expect(
        format!("{0} not found.", &filename).as_str());

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file.");

    return Vec::from_iter(contents.lines().map(String::from));
}
