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

fn solve(num_items: usize, max_weights: usize, items: &Vec<Vec<usize>>) -> () {
    let mut dp_tables: Vec<Vec<usize>> = (0..=num_items).map(|_| (0..=max_weights).map(|_| 0).collect()).collect();

    #[allow(unused_doc_comments)]
    /**             | d_(i, j)
     * d_(i+1, j) = | d_(i, j-w_(i+1)) + v_(i+1)   (j >= w_(i+1))
     *              | d_(i+1, j-w_(i+1)) + v_(i+1) (j >= w_(i+1))
     */
    for i in 0..num_items {
        for w in 0..=max_weights {
            if w >= items[i][1] {
                dp_tables[i + 1][w] = std::cmp::max(
                    dp_tables[i][w],
                    std::cmp::max(
                        dp_tables[i][w - items[i][1]] + items[i][0],
                        dp_tables[i + 1][w - items[i][1]] + items[i][0]
                    )
                );
            } else {
                dp_tables[i + 1][w] = dp_tables[i][w];
            }
        }
    }
    println!("{:?}", dp_tables[num_items][max_weights]);
}

fn main() {
    let (num_items, max_weights): (usize, usize) = (read(), read());
    let items: Vec<Vec<usize>> = (0..num_items).map(|_| (0..2).map(|_| read::<usize>()).collect()).collect();

    solve(num_items, max_weights, &items);
}