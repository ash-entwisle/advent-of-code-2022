
pub fn run(data: &String) -> i32 {
    data.lines()
        .map(|line: &str| line.split(|c| c == ',' || c == '-').collect())
        .map(|split: Vec<&str>| split.iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .map(|v: Vec<i32>| (v[0], v[1], v[2], v[3]))
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count() as i32
}


