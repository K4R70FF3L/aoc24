use regex::Regex;
use std::fs;

fn main() {
    run().expect("");
}

fn run() -> Result<(), String> {
    let input =
        fs::read_to_string("./res/input.txt").expect("Should have been able to read the file");
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();
    let line_regex = Regex::new(r"([0-9]+)[ ]+([0-9]+)").unwrap();
    for (_, [item1, item2]) in line_regex.captures_iter(&input).map(|c| c.extract()) {
        list1.push(item1.parse::<i64>().map_err(|e| e.to_string())?);
        list2.push(item2.parse::<i64>().map_err(|e| e.to_string())?);
    }
    list1.sort();
    list2.sort();

    let mut result = 0;

    for (item1, item2) in list1.iter().zip(list2.iter()) {
        result += (item1 - item2).abs();
    }
    println!("Part 1:");
    println!("{}", result);

    result = 0;
    for item1 in list1.iter() {
        let mut current_count: i64 = 0;
        for item2 in list2.iter() {
            if item1 == item2 {
                current_count += 1;
            }
        }
        result += item1 * current_count;
    }
    println!("Part 2:");
    println!("{}", result);

    return Ok(());
}
