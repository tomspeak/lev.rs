#[inline]
pub fn lev(s: &str, t: &str) -> usize {
    let n = s.len();
    let m = t.len();

    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }

    let min = |a: usize, b: usize, c: usize| -> usize { std::cmp::min(a, std::cmp::min(b, c)) };

    let mut grid = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        grid[i][0] = i;
    }

    for j in 0..=m {
        grid[0][j] = j;
    }

    for i in 1..=n {
        let sc = s.chars().nth(i - 1).unwrap_or('\0');

        for j in 1..=m {
            let tc = t.chars().nth(j - 1).unwrap_or('\0');

            let cost = if sc == tc { 0 } else { 1 };

            let above = grid[i - 1][j] + 1;
            let left = grid[i][j - 1] + 1;
            let diag = grid[i - 1][j - 1] + cost;

            let min_dist = min(above, left, diag);

            grid[i][j] = min_dist;
        }
    }

    grid[n][m] as usize
}

#[inline]
pub fn pretty_print(grid: &Vec<Vec<usize>>, s: &str, t: &str) {
    let t_chars: Vec<char> = t.chars().collect();

    print!("{:6}", "");
    for c in &t_chars {
        print!("{:1}  ", c);
    }
    println!();

    for (i, row) in grid.iter().enumerate() {
        let s_char = if i > 0 { s.chars().nth(i - 1) } else { None };
        if let Some(c) = s_char {
            print!("{:1}  ", c);
        } else {
            print!("{:3}", "");
        }

        for &value in row {
            print!("{:1}  ", value);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(lev("honda", "hyundai"), 3);
        assert_eq!(lev("sittmg", "setting"), 3);
        assert_eq!(lev("tom", "tom"), 0);
        assert_eq!(lev("gambol", "gumbo"), 2);
        assert_eq!(lev("kelm", "hello"), 3);
        assert_eq!(lev("christ", "christmas"), 3);
        assert_eq!(lev("🍆", "🍆"), 0);
        assert_eq!(lev("🤠", "🍆"), 1);
        assert_eq!(lev("你好", "你好"), 0);
        assert_eq!(lev("你好", "你"), 3);
        assert_eq!(lev("Измены", "Измены"), 0);
        assert_eq!(lev("Имн", "Измены"), 6);
    }
}

// @TODO: benchmark if this is faster
// use std::u8;
//
// pub fn lev(s: &str, t: &str) -> u8 {
//
//   let n = s.len() as u8;
//   let m = t.len() as u8;
//
//   // Use u8 for grid
//   let mut grid = vec![vec![0; m + 1]; n + 1];
//
//   // Update types of indices
//   for i in 0..=n {
//     grid[i as usize][0] = i;
//   }
//
//   for j in 0..=m {
//     grid[0][j as usize] = j;
//   }
//
//   // Update loop variables
//   for i in 1..=n {
//     let sc = s.chars().nth(i - 1).unwrap_or(' ');
//
//     for j in 1..=m {
//       let tc = t.chars().nth(j - 1).unwrap_or(' ');
//
//       let cost = if sc == tc { 0 } else { 1 };
//
//       let above = grid[i as usize - 1][j as usize] + 1;
//       let left = grid[i as usize][j as usize - 1] + 1;
//       let diag = grid[i as usize - 1][j as usize - 1] + cost;
//
//       let min_dist = min(above, left, diag);
//
//       grid[i as usize][j as usize] = min_dist;
//     }
//   }
//
//   // Return u8 distance
//   grid[n as usize][m as usize] as u8
// }
//
