use std::collections::HashMap;

fn main() {
    let data = include_str!("day14.txt");
    let mut and_mask: u64 = 0xFFFFFFFFFFFFFFFF;
    let mut or_mask: u64 = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in data.lines() {
        let mut iter = line.split(" = ");
        if line.starts_with("mask") {
            let mask = iter.nth(1).unwrap();
            let or_mask_s = mask.replace("X", "0");
            let and_mask_s = mask.replace("X", "1");
            or_mask = u64::from_str_radix(&or_mask_s, 2).unwrap();
            and_mask = u64::from_str_radix(&and_mask_s, 2).unwrap();
        } else {
            assert!(line.starts_with("mem"));
            let address_full = iter.next().unwrap();
            let address = &address_full[4..address_full.len() - 1]
                .parse::<u64>()
                .unwrap();
            let value = iter.next().unwrap().parse::<u64>().unwrap();
            mem.insert(*address, value & and_mask | or_mask);
        }
    }
    let sum: u64 = mem.values().sum();
    println!("{}", sum);
}
