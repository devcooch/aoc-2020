fn uid(n: &str) -> u32 {
    n.chars()
        .map(|x| (x == 'B' || x == 'R') as u32)
        .fold(0, |acc, x| acc * 2 + x)
}

fn main() {
    let data = include_str!("day05.txt");
    println!("{}", data.lines().map(|x| uid(x)).max().unwrap());
}
