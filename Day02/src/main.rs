use std::fs;

#[derive(Eq, PartialEq)]
enum Sorting {
    Asc,
    Desc,
}

struct Change {
    amount: i64,
    direction: Sorting,
}

const ALLOWED_DEVIATIONS: i64 = 1;

fn main() {
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
        if is_report_safe2(report, 0) {
            result += 1;
        }
    }
    println!("{}", result);
}

fn is_report_safe2(report: &Vec<i64>, change_count: i64) -> bool {
    if change_count > ALLOWED_DEVIATIONS {
        return false;
    }

    let changes = report
        .iter()
        .enumerate()
        .filter(|(index, _)| index + 1 < report.len())
        .map(|(index, item)| Change {
            amount: (report.get(index + 1).unwrap() - item).abs(),
            direction: if report.get(index + 1).unwrap() - item > 0 {
                Sorting::Asc
            } else {
                Sorting::Desc
            },
        })
        .collect::<Vec<Change>>();

    let mut asc_balance = 0;

    for (index, change) in changes.iter().enumerate() {
        if change.amount < 1 || change.amount > 3 {
            return is_report_safe2(
                &report
                    .iter()
                    .enumerate()
                    .filter(|(report_index, _)| index + 1 != *report_index)
                    .map(|(_, item)| *item)
                    .collect::<Vec<i64>>(),
                change_count + 1,
            ) || is_report_safe2(
                &report
                    .iter()
                    .enumerate()
                    .filter(|(report_index, _)| index != *report_index)
                    .map(|(_, item)| *item)
                    .collect::<Vec<i64>>(),
                change_count + 1,
            );
        }

        match change.direction {
            Sorting::Asc => asc_balance += 1,
            Sorting::Desc => asc_balance -= 1,
        }
    }

    let overall_direction;

    if asc_balance > 0 {
        overall_direction = Sorting::Asc;
    } else if asc_balance < 0 {
        overall_direction = Sorting::Desc;
    } else {
        return false;
    }

    for (index, change) in changes.iter().enumerate() {
        if overall_direction != change.direction {
            return is_report_safe2(
                &report
                    .iter()
                    .enumerate()
                    .filter(|(report_index, _)| index + 1 != *report_index)
                    .map(|(_, item)| *item)
                    .collect::<Vec<i64>>(),
                change_count + 1,
            ) || is_report_safe2(
                &report
                    .iter()
                    .enumerate()
                    .filter(|(report_index, _)| index != *report_index)
                    .map(|(_, item)| *item)
                    .collect::<Vec<i64>>(),
                change_count + 1,
            );
        }
    }

    return true;
}

fn is_report_safe(report: &Vec<i64>) -> bool {
    let direction;
    if report.get(0) < report.get(1) {
        direction = Sorting::Asc;
    } else if report.get(0) > report.get(1) {
        direction = Sorting::Desc;
    } else {
        return false;
    }
    let mut index = 1;
    while index < report.len() {
        let mut difference = report.get(index).unwrap() - report.get(index - 1).unwrap();

        match direction {
            Sorting::Desc => difference *= -1,
            _ => (),
        }

        if difference < 1 || difference > 3 {
            return false;
        }

        index += 1;
    }
    return true;
}
