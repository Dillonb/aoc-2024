use std::fs;


fn main() {
    let matrix : Vec<Vec<char>> = fs::read_to_string("data/day4/input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let coords_valid = |x: i32, y: i32| {
        return x >= 0
            && y >= 0
            && (y as usize) < matrix.len()
            && (x as usize) < matrix[y as usize].len();
    };

    let char_at = |x: i32, y: i32, c: char| {
        return coords_valid(x, y) && c == matrix[y as usize][x as usize];
    };

    let find_word = |x: i32, y: i32, dx: i32, dy: i32, word: &str| {
        let mut xi = x;
        let mut yi = y;

        for c in word.chars() {
            if !char_at(xi, yi, c) {
                return false;
            }

            xi += dx;
            yi += dy;
        }
        return true;
    };

    let directions = [
        (1,   0), // right
        (1,   1), // right down
        (0,   1), // down
        (-1,  1), // left down
        (-1,  0), // left
        (-1, -1), // left up
        (0,  -1), // up
        (1,  -1), // right up
    ];

    let mut num_xmas = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 'X' {
                for (dx, dy) in directions {
                    if find_word(x as i32, y as i32, dx, dy, "XMAS") {
                        num_xmas += 1;
                    }
                }
            }
        }
    }

    let x_mas_pattern_offsets = [
        (-1, -1),
        (-1,  1),
        (1,  -1),
        (1,   1)
    ];
    let x_mas_patterns = [
        // M.M
        // .A.
        // S.S
        ['M', 'M', 'S', 'S'],
        // S.S
        // .A.
        // M.M
        ['S', 'S', 'M', 'M'],
        // M.S
        // .A.
        // M.S
        ['M', 'S', 'M', 'S'],
        // S.M
        // .A.
        // S.M
        ['S', 'M', 'S', 'M']
    ];

    let mut num_x_mas = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 'A' {
                if x_mas_patterns.iter().any(|pattern| {
                    return pattern
                        .iter()
                        .zip(x_mas_pattern_offsets)
                        .all(|(c, (dx, dy))| {
                            return char_at(x as i32 + dx, y as i32 + dy, *c);
                    });
                }) {
                    num_x_mas += 1;
                }
            }
        }
    }

    println!("Num xmas: {}", num_xmas);
    println!("Num x-mas: {}", num_x_mas);
}
