#[derive(Clone, PartialEq)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone)]
struct Instruction {
    opc: OpCode,
    val: i64,
}

fn execute(dp: &[Instruction]) -> (bool, i64) {
    let mut acc = 0i64;
    let mut ip = 0usize;
    let mut visited = vec![false; dp.len()];
    while ip < dp.len() && !visited[ip] {
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
    (ip >= dp.len(), acc)
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
    for i in 0..dp.len() {
        if dp[i].opc == OpCode::Acc {
            continue;
        }
        let mut test = dp.to_vec();
        test[i].opc = match dp[i].opc {
            OpCode::Jmp => OpCode::Nop,
            OpCode::Nop => OpCode::Jmp,
            _ => panic!("Uknown opcode"),
        };
        let (success, acc) = execute(&test);
        if success {
            println!("{}", acc);
            break;
        }
    }
}
