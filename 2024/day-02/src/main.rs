use std::fs;

fn main() {
    let file = fs::read_to_string("in.txt").unwrap();
    part1(&file);
    part2(&file);
}

fn is_save(report: &Vec<i32>) -> bool {
    let is_ascending = |vec: &Vec<i32>| -> bool {
        vec.windows(2).all(|win| win[0] <= win[1] && (win[1] - win[0]) >= 1 && (win[1] - win[0]) <= 3)
    };
    let is_descending = |vec: &Vec<i32>| -> bool {
        vec.windows(2).all(|win| win[0] >= win[1] && (win[0] - win[1]) >= 1 && (win[0] - win[1]) <= 3)
    };

    is_descending(report) || is_ascending(report)
}

fn part1(file: &String) {
    let mut safe: i32 = 0;
    for line in file.lines() {
        let mut report: Vec<i32> = Vec::new();

        for word in line.split_whitespace() {
            let level = word.parse::<i32>().unwrap();
            report.push(level);
        }

        if is_save(&report) {
            safe += 1;
        }
    }

    println!("Part 1: {}", safe);
}

fn part2(file: &String) {
    let mut safe: i32 = 0;
    for line in file.lines() {
        let mut report: Vec<i32> = Vec::new();

        for word in line.split_whitespace() {
            let level = word.parse::<i32>().unwrap();
            report.push(level);
        }

        if is_save(&report) {
            safe += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if is_save(&new_report) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2: {}", safe);
}