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

fn travel(sx: i32, sy: i32, swx: i32, swy: i32, ins: &[(char, i32)]) -> (i32, i32) {
    let mut x = sx;
    let mut y = sy;
    let mut wx = swx;
    let mut wy = swy;
    for (command, value) in ins {
        match command {
            'N' => wy += value,
            'S' => wy -= value,
            'E' => wx += value,
            'W' => wx -= value,
            'F' => {
                x += value * wx;
                y += value * wy;
            }
            'L' => {
                let (ox, oy) = (wx, wy);
                wx = ox * cos(*value) - oy * sin(*value);
                wy = oy * cos(*value) + ox * sin(*value);
            }
            'R' => {
                let (ox, oy) = (wx, wy);
                wx = ox * cos(*value) + oy * sin(*value);
                wy = oy * cos(*value) - ox * sin(*value);
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
    let (x, y) = travel(0, 0, 10, 1, &ins);
    println!("{}", x.abs() + y.abs());
}
