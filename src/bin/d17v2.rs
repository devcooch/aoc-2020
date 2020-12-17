use std::collections::HashSet;

fn get_neighbors(x: isize, y: isize, z: isize, w: isize) -> Vec<(isize, isize, isize, isize)> {
    let mut res = Vec::new();
    res.reserve(81);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    res.push((x + dx, y + dy, z + dz, w + dw));
                }
            }
        }
    }
    res.remove(40);
    res
}

fn main() {
    let z0 = include_str!("day17.txt");
    let mut active = HashSet::<(isize, isize, isize, isize)>::new();
    let null = (z0.lines().take(1).next().unwrap().len() / 2) as isize + 1;
    for (row, line) in z0.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((column as isize - null, row as isize - null, 0, 0));
            }
        }
    }
    for step in 0isize..7 {
        println!("Step {}: {}", step, active.len());
        let mut to_deactivate = Vec::new();
        let mut to_activate = Vec::new();
        for x in -null - step..=null + step {
            for y in -null - step..=null + step {
                for z in -step - 1..=step + 1 {
                    for w in -step - 1..=step + 1 {
                        let active_n_count: usize = get_neighbors(x, y, z, w)
                            .iter()
                            .map(|n| active.contains(n) as usize)
                            .sum();
                        if active.contains(&(x, y, z, w))
                            && (active_n_count < 2 || active_n_count > 3)
                        {
                            to_deactivate.push((x, y, z, w));
                        }
                        if active_n_count == 3 {
                            to_activate.push((x, y, z, w));
                        }
                    }
                }
            }
        }
        for p in to_deactivate {
            active.remove(&p);
        }
        for p in to_activate {
            active.insert(p);
        }
    }
}
