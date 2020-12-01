use std::collections::HashSet;

fn main() {
    let contents = include_str!("day01.txt");
    let nums: HashSet<i64> = contents
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    'outer: for n in nums.iter() {
        for x in nums.iter() {
            if nums.contains(&(2020 - n - x)) {
                println!("{}", n * x * (2020 - n - x));
                break 'outer;
            }
        }
    }
}
