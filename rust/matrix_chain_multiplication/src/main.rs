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
    let n: usize = read();
    let matrix: Vec<Vec<usize>> = (0..n).map(|_| (0..2).map(|_| read::<usize>()).collect()).collect();
    let mut dp_table: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n]; n];

    for i in 0..n {
        dp_table[i][i] = 0;
    }

    for l in 1..n {
        for i in 0..n - l {
            let j: usize = i + l;
            for k in i..j {
                dp_table[i][j] = std::cmp::min(
                    dp_table[i][j],
                    dp_table[i][k] + dp_table[k + 1][j] + matrix[i][0] * matrix[k][1] * matrix[j][1],
                );
            }
        }
    }

    println!("{:?}", dp_table[0][n - 1]);
}
