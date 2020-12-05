fn uid(n: &str) -> usize {
    n.chars()
        .map(|x| (x == 'B' || x == 'R') as usize)
        .fold(0, |acc, x| acc * 2 + x)
}

fn main() {
    let data = include_str!("day05.txt");
    let mut nums = data.lines().map(|x| uid(x)).collect::<Vec<_>>();
    nums.sort_unstable();
    let delta = nums[0];
    let seat = nums
        .iter()
        .enumerate()
        .find(|(i, &x)| x - *i != delta)
        .unwrap()
        .1
        - 1;
    println!("{}", seat);
}
