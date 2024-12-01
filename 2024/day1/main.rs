use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn day_1_part_1(file: BufReader<File>) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    let _: () = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut x = line.split_whitespace();
            left.push(x.next().unwrap().parse().unwrap());
            right.push(x.next().unwrap().parse().unwrap());
        })
        .collect();
    left.sort();
    right.sort();

    let mut c = 0;
    let mut distance = 0;

    while c < left.len() {
        if left[c] > right[c] {
            distance += left[c] - right[c];
        } else if right[c] > left[c] {
            distance += right[c] - left[c];
        }

        c += 1
    }
    return distance;
}
fn day_1_part_2(file: BufReader<File>) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    let _: () = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut x = line.split_whitespace();
            left.push(x.next().unwrap().parse().unwrap());
            right.push(x.next().unwrap().parse().unwrap());
        })
        .collect();
    left.sort();
    right.sort();

    let mut c = 0;
    let mut simularty = 0;
    while c < left.len() {
        let count: u32 = right
            .iter()
            .filter(|f| **f == left[c])
            .count()
            .try_into()
            .unwrap();
        simularty += (left[c] * count);
        c += 1
    }

    return simularty;
}
fn main() {
    let file = File::open("input").unwrap();
    let file = BufReader::new(file);
    println!("Total distance is: {}", day_1_part_1(file));
    let file = File::open("input").unwrap();
    let file = BufReader::new(file);
    println!("Total Simultry is: {}", day_1_part_2(file))
}
