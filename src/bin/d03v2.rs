fn main() {
    let data = include_str!("day03.txt");
    let dxs = vec![1, 3, 5, 7, 1];
    let dys = vec![1, 1, 1, 1, 2];
    let map: Vec<Vec<bool>> = data
        .lines()
        .map(|x| x.chars().map(|y| y == '#').collect())
        .collect();
    let w = map[0].len();
    let mut result = 1u64;
    for (dx, dy) in dxs.iter().zip(dys.iter()) {
        let total: u64 = map
            .iter()
            .enumerate()
            .step_by(*dy)
            .map(|(i, x)| x[dx * i / dy % w] as u64)
            .sum();
        result *= total;
    }
    println!("{}", result);
}
