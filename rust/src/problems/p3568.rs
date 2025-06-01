pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let (m, n, energy) = (
            classroom.len() as i16,
            classroom[0].len() as i16,
            energy as i8,
        );
        let mut start = 0;
        let mut count = 0;
        let mut liters = vec![0; (m * n) as usize];
        for (i, r) in classroom.iter().enumerate() {
            for (j, &c) in r.as_bytes().iter().enumerate() {
                let pos = i as i16 * n + j as i16;
                match c {
                    b'L' => {
                        liters[pos as usize] = 1 << count;
                        count += 1;
                    }
                    b'S' => start = pos,
                    _ => {}
                }
            }
        }
        let all = (1u16 << count) - 1;
        if all == 0 {
            return 0;
        }
        let mut q = VecDeque::with_capacity((m * n) as usize);
        let mut vis = vec![vec![-1; all as usize + 1]; (m * n) as usize];
        q.push_back((0, start, energy));
        vis[start as usize][0] = energy;
        let mut dist = 0;
        while !q.is_empty() {
            let k = q.len();
            for _ in 0..k {
                let (mask, pos, e) = q.pop_front().unwrap();
                if mask == all {
                    return dist;
                }
                if e == 0 {
                    continue;
                }
                let (i, j) = (pos / n, pos % n);
                for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                    if i1 < 0 || j1 < 0 || i1 == m || j1 == n {
                        continue;
                    }
                    let cell = classroom[i1 as usize].as_bytes()[j1 as usize];
                    if cell == b'X' {
                        continue;
                    }
                    let mut e1 = e - 1;
                    let mut m1 = mask;
                    let pos1 = i1 * n + j1;
                    if cell == b'R' {
                        e1 = energy;
                    } else if cell == b'L' {
                        m1 |= liters[pos1 as usize];
                    }
                    if e1 > vis[pos1 as usize][m1 as usize] {
                        vis[pos1 as usize][m1 as usize] = e1;
                        q.push_back((m1, pos1, e1));
                    }
                }
            }

            dist += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let classroom = ["S.", "XL"].iter().map(|s| s.to_string()).collect();
        assert_eq!(2, Solution::min_moves(classroom, 2));
    }

    #[test]
    fn case2() {
        let classroom = ["LS", "RL"].iter().map(|s| s.to_string()).collect();
        assert_eq!(3, Solution::min_moves(classroom, 4));
    }

    #[test]
    fn case3() {
        let classroom = ["L.S", "RXL"].iter().map(|s| s.to_string()).collect();
        assert_eq!(-1, Solution::min_moves(classroom, 3));
    }
}
