use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("in.txt").unwrap();
    part1(&file);
    part2(&file);
}

fn part1(file: &String) {
    let mut list1:Vec<i32> = Vec::new();
    let mut list2:Vec<i32> = Vec::new();

    for line in file.lines() {
        let mut locations = line.split_whitespace();
        list1.push(locations.next().unwrap().parse::<i32>().unwrap());
        list2.push(locations.next().unwrap().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let result: i32 = std::iter::zip(list1, list2)
        .map(|(l1, l2)| (l1 - l2).abs())
        .sum();

    println!("Part 1: {}", result)
}

fn part2(file: &String) {
    let mut list1: HashMap<i32, i32> = HashMap::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in file.lines() {
        let mut locations = line.split_whitespace();
        let item1 = locations.next().unwrap().parse::<i32>().unwrap();
        let item2 = locations.next().unwrap().parse::<i32>().unwrap();
        list2.push(item2);

        if list1.contains_key(&item1) {
            continue;
        }
        list1.insert(item1, 0);
    }

    for n in list2 {
        if list1.contains_key(&n) {
            let counter = list1.get(&n).unwrap() + 1;
            list1.insert(n, counter);
        }
    }

    let result: i32 = list1.iter().map(|(k, v)| k * v).sum();
    println!("Part 2: {}", result);
}
