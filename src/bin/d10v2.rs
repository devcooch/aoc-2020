fn options_in_range(r: &[usize]) -> usize {
    match r.len() {
        0 | 1 | 2 => 1,
        3 => 2,
        4 => 4, /* 0, 1, 2, 3 | 0, 3 | 0, 1, 3 | 0, 2, 3 */
        5 => 7, /* 0, 1, 2, 3, 4 | 0, 2, 3, 4 | 0, 3, 4 | 0, 1, 2, 4 | 0, 1, 4 | 0, 2, 4 | 0, 1, 3, 4 */
        _ => panic!("WUT!"),
    }
}

fn main() {
    let mut data: Vec<_> = include_str!("day10.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    data.push(0);
    data.sort_unstable();
    let mut result = 1;
    let mut prev = 0;
    let mut start = 0;
    for (i, x) in data.iter().enumerate() {
        if x - prev == 3 {
            let range = start..i;
            result *= options_in_range(&data[range]);
            start = i;
        }
        prev = *x;
    }
    let range = start..;
    result *= options_in_range(&data[range]);
    println!("{}", result);
}
