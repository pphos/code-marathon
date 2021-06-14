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
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n]; n];

    for i in (0..n - 1).rev() {
        for j in (i + 1)..n {
            dp[i][j] = (i..j).map(|k| dp[i][k] + dp[k + 1][j] + matrix[i][0] * matrix[k][1] * matrix[j][1]).min().unwrap();
        }
    }

    println!("{:?}", dp[0][n - 1])
}
