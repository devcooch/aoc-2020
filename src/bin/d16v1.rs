fn merge_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
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
    let mut lines = data.lines();
    let mut ranges = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let ranges_line = line.split(": ").nth(1).unwrap();
        for range_s in ranges_line.split(" or ") {
            let mut range_it = range_s.split('-').map(|x| x.parse::<usize>().unwrap());
            ranges.push((range_it.next().unwrap(), range_it.next().unwrap()));
        }
    }
    ranges.sort_unstable_by_key(|k| k.0);
    let merged = merge_ranges(&ranges);
    assert!(merged.len() == 1);
    let (l, r) = merged[0];
    let total: usize = lines
        .skip(4)
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .filter(|x| x < &l || x > &r)
                .sum::<usize>()
        })
        .sum();
    println!("{}", total);
}
