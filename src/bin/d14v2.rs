use std::collections::HashMap;

fn bits(x: u64) -> Vec<u64> {
    let mut result = vec![0u64; 64];
    for (n, it) in result.iter_mut().enumerate() {
        *it = (x >> n) & 0x01;
    }
    result
}

fn prepare_masks(template: &str, xxx: u64) -> (u64, u64) {
    let bits = bits(xxx);
    let mut n = 0;
    let mut or_mask = 0u64;
    for c in template.chars() {
        or_mask <<= 1;
        match c {
            'X' => {
                or_mask += bits[n];
                n += 1;
            }
            _ => or_mask += (c as u64) - ('0' as u64),
        }
    }
    n = 0;
    let mut and_mask = 0;
    for c in template.chars() {
        and_mask <<= 1;
        if c == 'X' {
            and_mask += bits[n];
            n += 1;
        } else {
            and_mask += 1;
        }
    }
    (and_mask, or_mask)
}

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
            let n = 2u64.pow(mask.chars().filter(|x| x == &'X').count() as u32);
            for i in 0..n {
                let (and_mask, or_mask) = prepare_masks(mask, i);
                and_masks.push(and_mask);
                or_masks.push(or_mask);
            }
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
