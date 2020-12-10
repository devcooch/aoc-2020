fn main() {
    let mut data: Vec<_> = include_str!("day10.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    data.sort_unstable();
    let mut counts = vec![0, 0, 0, 1]; // 1 for "3" as your adapter has +3
    let mut current = 0;
    for x in data {
        counts[x - current] += 1;
        current = x;
    }
    println!("{}", counts[1] * counts[3]);
}
