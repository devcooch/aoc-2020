enum OpCode {
    Nop,
    Acc,
    Jmp,
}

struct Instruction {
    opc: OpCode,
    val: i64,
}

fn execute(dp: &[Instruction]) -> i64 {
    let mut acc = 0i64;
    let mut ip = 0usize;
    let mut visited = vec![false; dp.len()];
    while !visited[ip] {
        visited[ip] = true;
        match &dp[ip].opc {
            OpCode::Nop => ip += 1,
            OpCode::Acc => {
                acc += dp[ip].val;
                ip += 1;
            }
            OpCode::Jmp => ip = (ip as i64 + dp[ip].val) as usize,
        }
    }
    acc
}

fn main() {
    let src = include_str!("day08.txt");
    let mut dp = Vec::new();
    for line in src.lines() {
        let mut iter = line.split(' ');
        let ops = iter.next().unwrap();
        let val = iter.next().unwrap().parse::<i64>().unwrap();
        let opc = match ops {
            "nop" => OpCode::Nop,
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => panic!("Unexpected command: {}", ops),
        };
        dp.push(Instruction { opc, val });
    }
    let acc = execute(&dp);
    println!("{}", acc);
}
