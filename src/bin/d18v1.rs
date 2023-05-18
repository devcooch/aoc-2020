#[derive(Debug)]
enum Op {
    Add,
    Multiply,
    Tbd,
}

#[derive(Debug)]
enum State {
    LH,
    RH,
    OP,
}

fn calc_op(lh: u64, op: Op, rh: u64) -> u64 {
    match op {
        Op::Tbd => panic!("TBD"),
        Op::Add => lh + rh,
        Op::Multiply => lh * rh,
    }
}

fn print_stack(s: &Vec<(u64, Op)>) {
    print!(" => ");
    for x in s {
        print!("[{} {:?}] ", x.0, x.1);
    }
    println!();
}

fn calculate(s: &str, i: usize) -> u64 {
    let mut stack: Vec<(u64, Op)> = Vec::new();
    let mut state = State::LH;
    for c in s.chars() {
        match (c, &state) {
            (' ', _) => continue,
            ('0'..='9', State::LH) => {
                stack.push((c.to_digit(10).unwrap() as u64, Op::Tbd));
                state = State::OP;
            }
            ('+', State::OP) => {
                state = State::RH;
                if let Some(v) = stack.last_mut() {
                    v.1 = Op::Add;
                };
            }
            ('*', State::OP) => {
                state = State::RH;
                if let Some(v) = stack.last_mut() {
                    v.1 = Op::Multiply;
                };
            }
            ('0'..='9', State::RH) => {
                assert!(!stack.is_empty());
                let rh = c.to_digit(10).unwrap() as u64;
                let (lh, op) = stack.pop().unwrap();
                let v = calc_op(lh, op, rh);
                stack.push((v, Op::Tbd));
                state = State::OP;
            }
            ('(', State::LH) => {
                state = State::LH;
                stack.push((0, Op::Add));
            }
            ('(', State::RH) => {
                state = State::LH;
            }
            (')', State::OP) => {
                let (rh, _) = stack.pop().unwrap();
                let (lh, op) = stack.pop().unwrap();
                let v = calc_op(lh, op, rh);
                stack.push((v, Op::Tbd));
                state = State::OP;
            }
            _ => panic!(
                "Unknown char/state combination: {}/{:?} (line {})",
                c,
                state,
                i + 1
            ),
        }
        if c != ' ' {
            //            println!("{}", c);
            //            print_stack(&stack);
        }
    }
    stack.last().unwrap().0
}

fn main() {
    let data = include_str!("day18.txt");
    let debug = 222;
    let total: u64 = data
        .lines()
        .enumerate()
        //        .filter(|(i, _)| i + 1 == debug)
        .inspect(|(i, line)| print!("{}: {}", i + 1, line))
        .map(|(i, line)| calculate(line, i))
        .inspect(|v| println!(" = {}", v))
        .sum();
    println!("{}", total);
}
