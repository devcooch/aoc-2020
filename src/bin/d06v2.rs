use std::collections::HashSet;

fn get_all() -> HashSet<char> {
    let mut result = HashSet::new();
    for c in 'a'..'z' {
        result.insert(c);
    }
    result.insert('z');
    result
}

fn main() {
    let data = include_str!("day06.txt");
    let mut cross = get_all();
    let mut total = 0;
    for line in data.lines() {
        if line.is_empty() {
            total += cross.len();
            cross = get_all();
        } else {
            let answers : HashSet<_> = line.chars().collect();
            cross = cross.intersection(&answers).cloned().collect();
        }
    }
    total += cross.len();
    println!("{}", total);
}
