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
    let n: u32 = read();
    let mut n_vec: Vec<u32> = (0..n).map(|_| read::<u32>()).collect();
    let mut result: u32 = 0;

    loop {
        match n_vec.iter().any(|&x| x % 2 != 0) {
            true => break,
            false => {
                n_vec = n_vec.iter().map(|&x| x / 2).collect();
                result += 1;
            },
        }
    }
    println!("{}", result);
}