fn options_in_range(r: &[usize]) -> usize {
    match r.len() {
        0 | 1 | 2 => 1,
        3 => 2,
        4 => 4,
        5 => 6,
        _ => panic!("WUT!"),
    }
    /*
    if r.len() < 3 {
        return 1;
    }
    let d = r.iter().zip(r.iter().skip(1)).map(|(l, r)| r - l).inspect(|x| print!("{} ", x)).collect::<Vec<usize>>();
    println!();
    d.len()
    */
}

fn main() {
    let mut data: Vec<_> = include_str!("day10.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
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
    println!("{}", result);
}
