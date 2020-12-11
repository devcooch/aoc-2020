fn main() {
    let mut layout: Vec<Vec<char>> = include_str!("day11.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let h = layout.len();
    let w = layout[0].len();
    for row in &mut layout {
        row.insert(0, '.');
        row.push('.');
    }
    layout.push(vec!['.'; w + 2]);
    layout.insert(0, vec!['.'; w + 2]);
    let mut changed = Vec::new();
    loop {
        for row in 1..=h {
            for col in 1..=w {
                let ch = layout[row][col];
                if ch == '.' {
                    continue;
                }
                let mut occupied = 0usize;
                for ny in row - 1..=row + 1 {
                    for nx in col - 1..=col + 1 {
                        if ny != row || nx != col {
                            occupied += (layout[ny][nx] == '#') as usize;
                        }
                    }
                }
                if (ch == 'L' && occupied == 0) || (ch == '#' && occupied >= 4) {
                    changed.push((row, col));
                }
            }
        }
        if changed.is_empty() {
            break;
        }
        for (row, col) in &changed {
            layout[*row][*col] = if layout[*row][*col] == 'L' { '#' } else { 'L' };
        }
        changed.clear();
    }
    let total: usize = layout
        .iter()
        .map(|l| l.iter().map(|&c| (c == '#') as usize).sum::<usize>())
        .sum();
    println!("{}", total);
}
