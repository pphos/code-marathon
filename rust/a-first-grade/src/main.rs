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
    const MAX: usize = 20;

    let n: usize = read();
    let nums: Vec<usize> = (0..n-1).map(|_| read::<usize>()).collect();
    let result: usize = read();

    let mut dp_tables: Vec<Vec<u64>> = (0..n-1).map(|_| (0..=MAX).map(|_| 0).collect()).collect();

    dp_tables[0][nums[0]] = 1;
    for i in 0..n-2 {
        for j in 0..=MAX {
            let added_num = (j as i32) + (nums[i] as i32);
            let subtracted_num = (j as i32) - (nums[i] as i32);

            if added_num <= MAX as i32 {
                dp_tables[i + 1][added_num as usize] += dp_tables[i][j]
            }

            if 0 <= subtracted_num {
                dp_tables[i + 1][subtracted_num as usize] += dp_tables[i][j];
            }
        }
    }

    println!("{}", dp_tables[n - 2][result]);
}