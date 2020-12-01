use std::collections::HashSet;

fn main() {
    let contents = include_str!("day01.txt");
    let nums: HashSet<i64> = contents
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    for n in nums.iter() {
        if nums.contains(&(2020 - n)) {
            println!("{}", n * (2020 - n));
            break;
        }
    }
}
