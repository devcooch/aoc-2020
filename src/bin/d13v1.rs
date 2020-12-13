fn main() {
    let data = include_str!("day13.txt");
    let mut lines = data.lines();
    let ts = lines.next().unwrap().parse::<usize>().unwrap();
    let buses: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let best = buses
        .iter()
        .map(|x| (ts / x + 1, x))
        .inspect(|(cycles, id)| {
            println!("Bus {} comes in {} cycles at {}.", id, cycles, *id * cycles)
        })
        .min_by_key(|(cycles, id)| *id * cycles - ts)
        .unwrap();
    println!("{:?}", best);
    let result = best.1 * (best.1 * best.0 - ts);
    println!("{}", result);
}
