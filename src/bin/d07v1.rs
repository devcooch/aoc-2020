use std::collections::{HashMap, HashSet};

struct Rules {
    colors: HashMap<String, usize>,
    rules: HashMap<usize, Vec<(usize, usize)>>,
}

impl Rules {
    fn add_color(&mut self, bag_color: &str) -> usize {
        if !self.colors.contains_key(bag_color) {
            let new_color = self.colors.len();
            self.colors.insert(bag_color.to_string(), new_color);
            self.rules.insert(new_color, Vec::new());
        }
        self.colors[bag_color]
    }
    fn add_rule(&mut self, bag: &str, contents: &str) {
        let bag_color = self.add_color(bag);
        for inside in contents.split(", ") {
            let words: Vec<_> = inside.split(' ').collect();
            assert!(words.len() == 4 || (words.len() == 3 && words[0] == "no"));
            if words.len() == 4 {
                assert!(words[3] == "bag" || words[3] == "bags");
                let count = words[0].parse::<usize>().unwrap();
                let color = words[1].to_string() + " " + words[2];
                let inside_color = self.add_color(&color);
                self.rules
                    .get_mut(&bag_color)
                    .unwrap()
                    .push((count, inside_color));
            } // no new rules if we have 3 words
        }
    }

    fn new() -> Self {
        Rules {
            colors: HashMap::new(),
            rules: HashMap::new(),
        }
    }

    fn can_contain(&self, check_color: &usize) -> HashSet<usize> {
        let mut result = HashSet::new();
        for (color, content) in self.rules.iter() {
            if content.iter().any(|(_, inside)| inside == check_color) {
                result.insert(*color);
            }
        }
        result
    }
}

fn main() {
    let data = include_str!("day07.txt");
    let mut rules = Rules::new();
    for line in data.lines() {
        let mut iter = line.trim().split(" bags contain ");
        let from = iter.next().unwrap();
        let contents = iter.next().unwrap().trim().trim_matches('.');
        assert!(!contents.is_empty());
        rules.add_rule(from, contents);
    }
    let shiny_gold = rules.add_color("shiny gold");
    let mut current = rules.can_contain(&shiny_gold);
    let mut total = 0;
    while !current.is_empty() {
        print!("{} -> (", current.len());
        total += current.len();
        let previous: HashSet<_> = current.iter().cloned().collect();
        current.clear();
        for color in previous {
            print!("{},", color);
            for x in rules.can_contain(&color) {
                current.insert(x);
            }
        }
        println!(")");
    }
    println!("{}", total);
}
