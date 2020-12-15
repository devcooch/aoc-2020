use std::collections::HashMap;

fn main() {
    let start = [15usize, 12, 0, 14, 3, 1];
    let targets = [2020usize, 30000000];
    for target in targets.iter() {
        let mut seen: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut stream = start.iter().cloned().collect::<Vec<_>>();
        for (i, x) in start.iter().enumerate() {
            seen.insert(*x, (i, i));
        }
        stream.reserve(*target);
        while stream.len() < *target {
            let last = stream.last().unwrap();
            if seen.contains_key(&last) {
                let d = seen[&last].0 - seen[&last].1;
                if seen.contains_key(&d) {
                    seen.insert(d, (stream.len(), seen[&d].0));
                } else {
                    seen.insert(d, (stream.len(), stream.len()));
                }
                stream.push(d);
            } else {
                seen.insert(0, (stream.len(), seen[&0].0));
                stream.push(0);
            }
        }
        println!("For target {} the value is {}", target, stream.last().unwrap());
    }
}
