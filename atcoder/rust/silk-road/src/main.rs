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
    let (num_cities, limit_day): (usize, usize) = (read(), read());
    let distances: Vec<usize> = (0..num_cities).map(|_| read::<usize>()).collect();
    let weather_conditions: Vec<usize> = (0..limit_day).map(|_| read::<usize>()).collect();
    let mut dp_tables: Vec<Vec<usize>> = (0..limit_day).map(|_| (0..num_cities).map(|_| std::usize::MAX).collect()).collect();

    // 1都市目の処理
    dp_tables[0][0] = distances[0] * weather_conditions[0];
    for day_index in 1..limit_day {
        let yesterday_tired: usize = dp_tables[day_index - 1][0];
        let today_tired: usize = distances[0] * weather_conditions[day_index];

        dp_tables[day_index][0] = std::cmp::min(yesterday_tired, today_tired);
    }

    // 2都市目以降の処理
    let mut progress: usize = 1;
    for city_index in 1..num_cities {
        let prev_city_tired: usize = dp_tables[progress - 1][city_index - 1];
        let today_tired: usize = distances[city_index] * weather_conditions[progress];
        dp_tables[progress][city_index] = prev_city_tired + today_tired;

        for day_index in progress + 1..limit_day {
            let yesterday_tired: usize = dp_tables[day_index - 1][city_index];
            let prev_city_tired: usize = dp_tables[day_index - 1][city_index - 1];
            let today_tired: usize = distances[city_index] * weather_conditions[day_index];

            dp_tables[day_index][city_index] = std::cmp::min(yesterday_tired, today_tired + prev_city_tired);
        }
        progress += 1;
    }

    println!("{:?}", dp_tables[limit_day - 1][num_cities - 1]);

}