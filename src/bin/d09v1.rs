fn can_be_sum_of_two(n: &usize, v: &[usize]) -> bool {
    for (i, x) in v.iter().enumerate() {
        for y in v[i + 1..].iter() {
            if *n == x + y {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let xmas = include_str!("day09.txt");
    let nums: Vec<usize> = xmas.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    for (i, n) in nums.iter().skip(25).enumerate() {
        if !can_be_sum_of_two(&n, &nums[i..i + 25]) {
            println!("{}", n);
            break;
        }
    }
}
