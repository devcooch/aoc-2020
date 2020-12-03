fn main() {
    let data = include_str!("day03.txt");
    let map: Vec<Vec<bool>> = data
        .lines()
        .map(|x| x.chars().map(|y| y == '#').collect())
        .collect();
    let w = map[0].len();
    let total: u32 = map
        .iter()
        .enumerate()
        .map(|(i, x)| x[3 * i % w] as u32)
        .sum();
    println!("{}", total);
}
