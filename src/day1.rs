use std::fs;

fn parse_line(line: &str) -> (i32, i32) {
    let mut split = line.split_whitespace();
    let left = split.next().unwrap().parse::<i32>().unwrap();
    let right = split.next().unwrap().parse::<i32>().unwrap();
    return (left, right);
}

fn get_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_line)
        .unzip();

    left.sort();
    right.sort();

    return (left, right);
}

fn main() {
    let (left, right) = get_input("data/day1/input");

    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("Part 1, sum: {}", sum);

    let similarity_score: i32 = left.into_iter().map(|item| item * right.iter().filter(|right_item| **right_item == item).count() as i32).sum();

    println!("Part 2, similarity score: {}", similarity_score);
}
