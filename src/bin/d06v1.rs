use std::collections::HashSet;

fn main() {
    let data = include_str!("day06.txt");
    let mut answers = HashSet::new();
    let mut total = 0;
    for line in data.lines() {
        if line.is_empty() {
            total += answers.len();
            answers.clear();
        }
        for c in line.chars() {
            answers.insert(c);
        }
    }
    total += answers.len();
    println!("{}", total);
}
