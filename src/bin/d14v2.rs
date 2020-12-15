use std::collections::HashMap;

fn main() {
    let data = include_str!("day14.txt");
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut and_masks: Vec<u64> = Vec::new();
    let mut or_masks: Vec<u64> = Vec::new();
    for line in data.lines() {
        let mut iter = line.split(" = ");
        if line.starts_with("mask") {
            and_masks.clear();
            or_masks.clear();
            let mask = iter.nth(1).unwrap();
            let set_mask_s = mask.replace("X", "0");
            let set_mask = u64::from_str_radix(&set_mask_s, 2).unwrap();
        } else {
            let address_full = iter.next().unwrap();
            let address = &address_full[4..address_full.len() - 1]
                .parse::<u64>()
                .unwrap();
            let value = iter.next().unwrap().parse::<u64>().unwrap();
            for (and_mask, or_mask) in and_masks.iter().zip(or_masks.iter()) {
                mem.insert(address & and_mask | or_mask, value);
            }
        }
    }
    let sum: u64 = mem.values().sum();
    println!("{}", sum);
}
