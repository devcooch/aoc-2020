fn sin(x: i32) -> i32 {
    match x {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!("Sin({}) is unknown!", x),
    }
}

fn cos(x: i32) -> i32 {
    match x {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("Cos({}) is unknown!", x),
    }
}

fn travel(sx: i32, sy: i32, sor: i32, ins: &[(char, i32)]) -> (i32, i32) {
    let mut x = sx;
    let mut y = sy;
    let mut o = sor;
    for (command, value) in ins {
        match command {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'F' => {
                x += value * sin(o);
                y += value * cos(o);
            }
            'L' => {
                o -= value;
                o += 360;
                o %= 360;
            }
            'R' => {
                o += value;
                o %= 360;
            }
            _ => panic!("Unknown command {}", command),
        }
    }
    (x, y)
}

fn main() {
    let ins = include_str!("day12.txt")
        .lines()
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i32>().unwrap()))
        .collect::<Vec<(char, i32)>>();
    let (x, y) = travel(0, 0, 90, &ins);
    println!("{}", x.abs() + y.abs());
}
