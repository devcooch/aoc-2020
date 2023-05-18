use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
enum RuleType {
    Char,
    List,
}

struct Rule {
    typ: RuleType,
    ch: char,
    subrules: Vec<Vec<usize>>,
}

fn traverse(r: &HashMap<usize, Rule>, start: usize) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    result
}

fn main() {
    let data = include_str!("day19.txt");
    let mut lines = data.lines();
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut lr = line.split(": ");
        let index = lr.next().unwrap().parse::<usize>().unwrap();
        let rule_s = lr.next().unwrap();
        let rule = if rule_s.starts_with('"') {
            Rule {
                typ: RuleType::Char,
                ch: rule_s.chars().nth(1).unwrap(),
                subrules: Vec::new(),
            }
        } else {
            Rule {
                typ: RuleType::List,
                ch: '\0',
                subrules: rule_s
                    .split(" | ")
                    .map(|s| {
                        s.split(" ")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>(),
            }
        };
        rules.insert(index, rule);
    }
    let messages: Vec<_> = lines.collect();
    let variants = traverse(&rules, 0);
    println!(
        "{}",
        messages
            .iter()
            .filter(|&m| variants.contains(&m.to_string()))
            .collect::<Vec<_>>()
            .len()
    );
}
