use regex::Regex;
use std::fs;

fn main() {
    run().expect("");
}

fn run() -> Result<(), String> {
    let input =
        fs::read_to_string("./res/sample.txt").expect("Should have been able to read the file");
    let line_regex = Regex::new(r"([0-9]+)[ ]+([0-9]+)").unwrap();
    for (_, [item1, item2]) in line_regex.captures_iter(&input).map(|c| c.extract()) {}

    let mut result = 0;

    println!("Part 1:");
    println!("{}", result);

    println!("Part 2:");
    result = 0;
    println!("{}", result);

    return Ok(());
}
