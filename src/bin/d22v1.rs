use std::collections::VecDeque;

fn main() {
    let data = include_str!("day22.txt");
    let mut p1_deck: VecDeque<_> = data
        .lines()
        .take_while(|l| !l.is_empty())
        .filter(|l| !l.starts_with("Player"))
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    let mut p2_deck: VecDeque<_> = data
        .lines()
        .skip_while(|l| !l.is_empty())
        .filter(|l| !l.starts_with("Player") && !l.is_empty())
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1 = p1_deck.pop_front().unwrap();
        let p2 = p2_deck.pop_front().unwrap();
        if p1 > p2 {
            p1_deck.push_back(p1);
            p1_deck.push_back(p2);
        } else {
            p2_deck.push_back(p2);
            p2_deck.push_back(p1);
        }
    }
    let winner = if p1_deck.is_empty() { p2_deck } else { p1_deck };
    let result: usize = winner
        .iter()
        .enumerate()
        .map(|(i, x)| (winner.len() - i) * x)
        .sum();
    println!("{}", result);
}
