fn main() {
    let data = include_str!("day13.txt");
    let mut lines = data.lines();
    lines.next();
    let buses: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .map(|(i, x)| (i as u64, x.parse::<u64>().unwrap()))
        .collect();
    let ratio = buses[0].1;
    let mut ts = 100000000000000;
    while ts % ratio != 0 {
        ts += 1;
    }
    // 67a = x + 0
    // 7b  = x + 1
    // 59c = x + 2
    // 61d = x + 3
    'main: loop {
        //println!("Trying {}", ts);
        for (bus_offset, bus_ratio) in &buses[1..] {
            //print!("Bus {} should have offset {} minutes..", bus_ratio, bus_offset);
            if (ts + bus_offset) % bus_ratio != 0 {
                //println!("failed!");
                ts += ratio;
                continue 'main;
            }
            //println!("passed.");
        }
        //println!("All good!");
        break;
    }
    println!("{}", ts);
}
