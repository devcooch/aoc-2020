use regex::Regex;

fn main() {
    let data = include_str!("day02.txt");
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
    let mut total = 0;
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let min = caps.name("min").unwrap().as_str().parse::<usize>().unwrap();
        let max = caps.name("max").unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps
            .name("letter")
            .unwrap()
            .as_str()
            .parse::<char>()
            .unwrap();
        let password = caps.name("password").unwrap().as_str();
        let count: usize = password.chars().map(|x| (x == letter) as usize).sum();
        if count >= min && count <= max {
            total += 1;
        }
    }
    println!("{}", total);
}
