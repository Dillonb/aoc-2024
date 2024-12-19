use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day3/input").unwrap();

    let re_mul_only = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let re_mul_do_dont = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:do\(\))|(?:don't\(\))").unwrap();

    // Part 1
    let sum_of_mults : i32 = re_mul_only.captures_iter(&input)
        .map(|c| c.extract::<2>().1.map(|i| i.parse::<i32>().unwrap()))
        .map(|[num1, num2]| num1 * num2)
        .sum();

    // Part 2
    let mut enabled = true;
    let mut sum_of_only_enabled_mults : i32 = 0;
    for m in re_mul_do_dont.captures_iter(&input) {
        let token = m.get(0).unwrap().as_str();
        if token == "do()" {
            enabled = true;
        } else if token == "don't()" {
            enabled = false;
        } else if enabled {
            sum_of_only_enabled_mults += (m.get(1).unwrap().as_str().parse::<i32>().unwrap()) * (m.get(2).unwrap().as_str().parse::<i32>().unwrap());
        }
    }

    println!("Sum of all mults: {}", sum_of_mults);
    println!("Sum of only enabled mults: {}", sum_of_only_enabled_mults);
}
