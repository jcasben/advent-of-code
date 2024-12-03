use std::fs;

fn main() {
    part1();
}

fn part1() {
    let file = fs::read_to_string("in.txt").unwrap();

    let is_ascending = |vec: &Vec<i32>| -> bool {
        vec.windows(2).all(|win| win[0] <= win[1] && (win[1] - win[0]) >= 1 && (win[1] - win[0]) <= 3)
    };
    let is_descending = |vec: &Vec<i32>| -> bool {
        vec.windows(2).all(|win| win[0] >= win[1] && (win[0] - win[1]) >= 1 && (win[0] - win[1]) <= 3)
    };

    let mut safe: i32 = 0;
    for line in file.lines() {
        let mut report: Vec<i32> = Vec::new();

        for word in line.split_whitespace() {
            let level = word.parse::<i32>().unwrap();
            report.push(level);
        }

        if is_ascending(&report) || is_descending(&report) {
            safe += 1;
        }
    }

    println!("Part 1: {}", safe);
}
