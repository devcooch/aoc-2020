use regex::Regex;

fn main() {
    let data = include_str!("day02.txt");
    let re =
        Regex::new(r"(?P<first>\d+)-(?P<second>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
    let mut total = 0;
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let first = caps
            .name("first")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let second = caps
            .name("second")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let letter = caps
            .name("letter")
            .unwrap()
            .as_str()
            .parse::<char>()
            .unwrap();
        let password = caps.name("password").unwrap().as_str();
        if (password.chars().nth(first - 1) == Some(letter))
            ^ (password.chars().nth(second - 1) == Some(letter))
        {
            total += 1;
        }
    }
    println!("{}", total);
}
