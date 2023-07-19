pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let (mut x, mut y, mut d) = (0, 0, 0);
        let mut result = 0;
        let obs = obstacles
            .into_iter()
            .map(|o| (o[0], o[1]))
            .collect::<HashSet<_>>();
        let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for c in commands {
            match c {
                -2 => d = (d + 3) % 4,
                -1 => d = (d + 1) % 4,
                1..=9 => {
                    let (dx, dy) = dir[d as usize];
                    for _ in 1..=c {
                        let x1 = x + dx;
                        let y1 = y + dy;
                        if obs.contains(&(x1, y1)) {
                            break;
                        }
                        x = x1;
                        y = y1;
                        result = result.max(x * x + y * y);
                    }
                }
                _ => {}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(25, Solution::robot_sim(vec![4, -1, 3], vec![]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            65,
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]])
        );
    }
}
