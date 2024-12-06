use regex::Regex;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("./res/input.txt").expect("Should have been able to read the file");
    let regex =
        Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Should compile to valid regex");
    let mut result = 0;
    for (_, [factor1, factor2]) in regex.captures_iter(&input).map(|c| c.extract()) {
        result += factor1.parse::<i64>().unwrap() * factor2.parse::<i64>().unwrap();
    }

    println!("Part 1:");
    println!("{result}");

    let mut result = 0;
    let mut apply = true;
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))()|(don't\(\)())")
        .expect("Should compile to valid regex");
    for (_, [match1, match2]) in regex.captures_iter(&input).map(|c| c.extract()) {
        if match1 == "do()" {
            apply = true;
        } else if match1 == "don't()" {
            apply = false;
        } else {
            if apply {
                result += match1.parse::<i64>().unwrap() * match2.parse::<i64>().unwrap();
            }
        }
    }

    println!("Part 2:");
    println!("{result}");
}
