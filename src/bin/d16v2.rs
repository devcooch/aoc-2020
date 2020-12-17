use std::collections::HashSet;

fn merge_ranges(ranges: &[&(usize, usize)]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let (mut l1, mut r1) = ranges[0];
    for (l2, r2) in ranges {
        if l2 <= &r1 {
            r1 = *r2;
        } else {
            result.push((l1, r1));
            l1 = *l2;
            r1 = *r2;
        }
    }
    result.push((l1, r1));
    result
}

fn main() {
    let data = include_str!("day16.txt");
    let mut fields = Vec::new();
    let mut ranges = Vec::new();
    let mut lines = data.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut line_split = line.split(": ");
        let field_name = line_split.next().unwrap();
        fields.push(field_name);
        let ranges_line = line_split.next().unwrap();
        let mut field_ranges = Vec::new();
        for range_s in ranges_line.split(" or ") {
            let mut range_it = range_s.split('-').map(|x| x.parse::<usize>().unwrap());
            field_ranges.push((range_it.next().unwrap(), range_it.next().unwrap()));
        }
        ranges.push(field_ranges);
    }
    let mut all_ranges = ranges.iter().flatten().collect::<Vec<_>>();
    all_ranges.sort_unstable_by_key(|k| k.0);
    let merged = merge_ranges(&all_ranges);
    assert!(merged.len() == 1);
    let (l, r) = merged[0];
    let tickets = lines
        .filter(|l| l.starts_with(char::is_numeric))
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|t| t.iter().all(|&f| f >= l && f <= r))
        .collect::<Vec<_>>();
    let mut all_fields = HashSet::new();
    for f in 0..fields.len() {
        all_fields.insert(f);
    }
    let mut candidates = vec![all_fields; fields.len()];
    for ticket in tickets.iter() {
        for (order_index, field) in ticket.iter().enumerate() {
            for (range_index, field_ranges) in ranges.iter().enumerate() {
                if field_ranges.iter().all(|(fl, fr)| field < fl || field > fr) {
                    candidates[order_index].remove(&range_index);
                }
            }
        }
    }
    while candidates.iter().any(|fs| fs.len() > 1) {
        let determined: Vec<(usize, usize)> = candidates
            .iter()
            .enumerate()
            .filter(|(_, f)| f.len() == 1)
            .map(|(i, f)| (i, *f.iter().next().unwrap()))
            .collect();
        for (i, d) in determined {
            for (j, f) in candidates.iter_mut().enumerate() {
                if i != j {
                    f.remove(&d);
                }
            }
        }
    }
    println!("{:?}", candidates);
    let departures = fields
        .iter()
        .enumerate()
        .filter(|(_, n)| n.starts_with("departure"))
        .map(|(i, _)| candidates[i].iter().next().unwrap())
        .collect::<HashSet<_>>();
    println!("{:?}", departures);
    println!("{:?}", tickets[0]);
    let result: usize = tickets[0]
        .iter()
        .enumerate()
        .filter(|(i, _)| departures.contains(i))
        .map(|(_, v)| v)
        .product();
    println!("{}", result);
}
