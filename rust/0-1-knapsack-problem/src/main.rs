use std::io::*;
use std::str::FromStr;
use std::cmp;

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
    let (num_item, max_weight): (usize, usize) = (read(), read());
    let items: Vec<Vec<usize>> = (0..num_item).map(|_| (0..2).map(|_| read::<usize>()).collect()).collect();
    let mut dp_arr: Vec<Vec<usize>> = (0..=num_item).map(|_| (0..=max_weight).map(|_| 0).collect()).collect();

    for i in 0..num_item {
        for w in 0..=max_weight {
            if w >= items[i][1] {
                dp_arr[i + 1][w] = cmp::max(dp_arr[i][w], dp_arr[i][w - items[i][1]] + items[i][0]);
            } else {
                dp_arr[i + 1][w] = dp_arr[i][w];
            }
        }
    }
    println!("{}", dp_arr[num_item][max_weight]);
}
