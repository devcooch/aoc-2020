fn main() {
    let data = include_str!("day03.txt");
    let dxs = vec![1, 3, 5, 7, 1];
    let dys = vec![1, 1, 1, 1, 2];
    let field: Vec<Vec<bool>> = data
        .lines()
        .map(|x| x.chars().map(|y| y == '#').collect())
        .collect();
    let w = field[0].len();
    let result: u64 = dxs
        .iter()
        .zip(dys.iter())
        .map(|(&dx, &dy)| {
            field
                .iter()
                .enumerate()
                .step_by(dy)
                .map(|(i, x)| x[dx * i / dy % w] as u64)
                .sum::<u64>()
        })
        .product();
    println!("{}", result);
}
