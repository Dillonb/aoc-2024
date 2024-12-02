use std::fs;

fn parse_line(line: &str) -> Vec<i32> {
    return line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap()).collect();
}

fn get_input(path: &str) -> Vec<Vec<i32>> {
    return fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();
}

fn is_safe(report: &Vec<i32>) -> bool {
    let differences = report
        .windows(2)
        .map(|transition| (transition[1] - transition[0]));

    let all_valid_decrease = differences.clone().into_iter().all(|difference| difference < 0 && difference > -4);
    let all_valid_increase = differences.clone().into_iter().all(|difference| difference > 0 && difference < 4);

    let result = all_valid_decrease || all_valid_increase;
    return result;
}

fn is_safe_part2(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true
    } else {
        for i in 0..report.len() {
            let mut temp = report.clone();
            temp.remove(i);
            if is_safe(&temp) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let input = get_input("data/day2/input");
    let num_safe_part1 = input.clone().into_iter().filter(is_safe).count();
    println!("Part 1, number of safe lines: {}", num_safe_part1);

    let num_safe_part2 = input.clone().into_iter().filter(is_safe_part2).count();
    println!("Part 2, number of safe lines: {}", num_safe_part2);
}
