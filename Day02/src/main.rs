use std::fs;

fn main() {
    run().expect("");
}

fn run() -> Result<(), String> {
    let input =
        fs::read_to_string("./res/input.txt").expect("Should have been able to read the file");

    let reports: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|r| r.parse::<i64>().expect("Wasn't able to parse integer"))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut result = 0;

    println!("Part 1:");
    for report in reports.iter() {
        if is_report_safe(report) {
            result += 1;
        }
    }
    println!("{}", result);

    println!("Part 2:");
    result = 0;
    for report in reports.iter() {
        if is_report_safe2(report) {
            result += 1;
        }
    }
    println!("{}", result);

    return Ok(());
}

fn is_report_safe2(report: &Vec<i64>) -> bool {
    let direction: &str;
    if report.get(0) < report.get(1) {
        direction = "ASC";
    } else if report.get(0) > report.get(1) {
        direction = "DESC";
    } else {
        return false;
    }
    let mut index = 1;
    while index < report.len() {
        let mut difference = report.get(index).unwrap() - report.get(index - 1).unwrap();
        if direction == "DESC" {
            difference *= -1;
        }

        if difference < 1 || difference > 3 {
            return false;
        }

        index += 1;
    }
    return true;
}

fn is_report_safe(report: &Vec<i64>) -> bool {
    let direction: &str;
    if report.get(0) < report.get(1) {
        direction = "ASC";
    } else if report.get(0) > report.get(1) {
        direction = "DESC";
    } else {
        return false;
    }
    let mut index = 1;
    while index < report.len() {
        let mut difference = report.get(index).unwrap() - report.get(index - 1).unwrap();
        if direction == "DESC" {
            difference *= -1;
        }

        if difference < 1 || difference > 3 {
            return false;
        }

        index += 1;
    }
    return true;
}
