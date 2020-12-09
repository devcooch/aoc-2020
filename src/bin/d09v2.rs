fn main() {
    let xmas = include_str!("day09.txt");
    let nums: Vec<usize> = xmas.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    let target = 29221323;
    'outer: for from in 0..nums.len() {
        for to in from + 1..nums.len() {
            let s: usize = nums[from..to + 1].iter().sum();
            if s > target {
                continue 'outer;
            }
            if s == target {
                let min = nums[from..to + 1].iter().min().unwrap();
                let max = nums[from..to + 1].iter().max().unwrap();
                println!("{}", min + max);
                break 'outer;
            }
        }
    }
}
