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
                let mut visible = 0usize;
                for (dy, dx) in &[
                    (-1i64, -1i64),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    let mut ny = row as i64;
                    let mut nx = col as i64;
                    ny += dy;
                    nx += dx;
                    while (ny > 0) && (ny <= h as i64) && (nx > 0) && (nx <= w as i64) && layout[ny as usize][nx as usize] == '.' {
                        ny += dy;
                        nx += dx;
                    }
                    visible += (layout[ny as usize][nx as usize] == '#') as usize;
                }
                if (ch == 'L' && visible == 0) || (ch == '#' && visible >= 5) {
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
    println!("{} seats", total);
}
