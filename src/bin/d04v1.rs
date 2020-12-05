use std::collections::HashSet;

fn check_fields(allowed: &HashSet<&str>, fields: &HashSet<&str>) -> bool {
    let diff: HashSet<_> = allowed.difference(&fields).collect();
    diff.is_empty() || (diff.len() == 1 && diff.contains(&"cid"))
}

fn main() {
    let allowed: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .cloned()
        .collect();
    let data = include_str!("day04.txt");
    let mut fields = HashSet::new();
    let mut valid = 0;
    for line in data.lines() {
        if line.is_empty() {
            if check_fields(&allowed, &fields) {
                valid += 1;
            }
            fields.clear()
        }
        for word in line.split_ascii_whitespace() {
            fields.insert(word.split(':').next().unwrap());
        }
    }
    if check_fields(&allowed, &fields) {
        valid += 1;
    }
    println!("{}", valid);
}
