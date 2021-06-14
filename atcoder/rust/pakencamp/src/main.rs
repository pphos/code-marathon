use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let flag_type: usize = 3;
    let (flag_width, flag_height): (usize, usize) = (read(), 5);
    let flag_combination: [(usize, usize); 3] = [(1, 2), (0, 2), (0, 1)];

    // それぞれの色で列を塗りつぶすために必要な塗り替え回数のVectorの作成
    let mut flag_counts: Vec<Vec<usize>> = vec![vec![flag_height; flag_type]; flag_width];
    for _ in 0..flag_height {
        let line: Vec<char> = read::<String>().chars().collect();
        for (index, color) in line.iter().enumerate() {
            match color {
                'R' => flag_counts[index][0] -= 1,
                'W' => flag_counts[index][1] -= 1,
                'B' => flag_counts[index][2] -= 1,
                _ => (),
            }
        }
    }

    // DP
    for time in 1..flag_width {
        for color in 0..flag_type {
            let (prev_flag_index_0, prev_flag_index_1): (usize, usize) = flag_combination[color];
            let prev_flag_0: usize = flag_counts[time - 1][prev_flag_index_0];
            let prev_flag_1: usize = flag_counts[time - 1][prev_flag_index_1];

            flag_counts[time][color] += std::cmp::min(prev_flag_0, prev_flag_1);
        }
    }

    println!("{:?}", flag_counts[flag_width - 1].iter().min().unwrap());
}
