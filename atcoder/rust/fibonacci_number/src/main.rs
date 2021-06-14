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
    let mut dp_arr: Vec<u32> = (0..=n).map(|_| 0).collect();

    if n < 2 {
        println!("{}", 1);
    } else {
        dp_arr[0] = 1;
        dp_arr[1] = 1;
        for i  in 2..=n {
            dp_arr[i] = dp_arr[i - 1] + dp_arr[i - 2];
        }
        println!("{}", dp_arr[n]);
    }
}
