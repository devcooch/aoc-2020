use std::collections::HashMap;

fn validate_pass(pass: &HashMap<&str, &str>) -> bool {
    let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let mut valid = true;
    let mut must = 0;
    for (field, value) in pass {
        match *field {
            "byr" => {
                let yr = value.parse::<u32>().unwrap();
                valid &= (yr >= 1920) && (yr <= 2002);
                must += 1;
            }
            "iyr" => {
                let yr = value.parse::<u32>().unwrap();
                valid &= (yr >= 2010) && (yr <= 2020);
                must += 1;
            }
            "eyr" => {
                let yr = value.parse::<u32>().unwrap();
                valid &= (yr >= 2020) && (yr <= 2030);
                must += 1;
            }
            "hgt" => {
                let i = value.find(char::is_alphabetic);
                if i.is_some() {
                    let hgt = value.get(..i.unwrap()).unwrap().parse::<u32>().unwrap();
                    let units = value.get(i.unwrap()..).unwrap();
                    match units {
                        "cm" => valid &= (hgt >= 150) && (hgt <= 193),
                        "in" => valid &= (hgt >= 59) && (hgt <= 76),
                        _ => valid = false,
                    }
                } else {
                    valid = false;
                }
                must += 1;
            }
            "hcl" => {
                valid &= value.chars().next().unwrap() == '#'
                    && value
                        .chars()
                        .any(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'));
                must += 1;
            }
            "ecl" => {
                valid &= valid_ecl.contains(value);
                must += 1;
            }
            "pid" => {
                if value.len() != 9 {
                    valid = false;
                }
                let id = value.parse::<u64>();
                if id.is_err() {
                    valid = false;
                }
                must += 1;
            }
            "cid" => (),
            _ => {
                valid = false;
            }
        }
    }
    valid && must == 7
}

fn main() {
    let data = include_str!("day04.txt");
    let mut pass = HashMap::new();
    let mut valid = 0;
    for line in data.lines() {
        if line.is_empty() {
            if validate_pass(&pass) {
                valid += 1;
            }
            pass.clear()
        }
        for word in line.split_ascii_whitespace() {
            let mut t = word.split(':');
            pass.insert(t.next().unwrap(), t.next().unwrap());
        }
    }
    if validate_pass(&pass) {
        valid += 1;
    }
    println!("{}", valid);
}
