fn main() {
    let data = include_str!("day13.txt");
    let buses: Vec<_> = data.lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .map(|(i, x)| (i as u64, x.parse::<u64>().unwrap()))
        .collect();
    let fastest = buses.iter().max_by_key(|(_, x)| x).unwrap();
    let ratio = fastest.1;
    let offset = fastest.0;
    let mut ts = 0; //100000000000000;
    while (ts + offset) % ratio != 0 {
        ts += 1;
    }
    // 67a = 7b - 1 = 59c - 2 = 61d - 3
    // 67a = 7b + 6 = 59c + 57 = 61d + 58
    //
    // 67a = x + 0 = 754018 = 67 * 11254
    // 7b  = x + 1 = 754019 = 7 * 107717
    // 59c = x + 2 = 754020 = 59 * 12780
    // 61d = x + 3 = 754021 = 61 * 12361
    'main: loop {
        println!("Trying {}", ts);
        for (bus_offset, bus_ratio) in &buses {
            print!("Bus {} should have offset {} minutes..", bus_ratio, bus_offset);
            if (ts + bus_offset) % bus_ratio != 0 {
                println!("failed!");
                ts += ratio;
                continue 'main;
            }
            println!("passed.");
        }
        println!("All good!");
        break;
    }
    println!("{}", ts);
}
