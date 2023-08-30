pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let max = (*forbidden.iter().max().unwrap() + a + b).max(x + b) as usize;
        let mut ban = vec![false; max + 1];
        for e in forbidden {
            ban[e as usize] = true;
        }
        let mut f = vec![[i32::MAX; 2]; max + 1];
        f[0][0] = 0;
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while let Some((y, pre)) = q.pop_front() {
            if y == x {
                return f[y as usize][pre];
            }
            if pre == 0
                && y - b >= 0
                && !ban[(y - b) as usize]
                && f[y as usize][pre] + 1 < f[(y - b) as usize][1]
            {
                f[(y - b) as usize][1] = f[y as usize][pre] + 1;
                q.push_back((y - b, 1));
            }
            if y + a <= max as i32
                && !ban[(y + a) as usize]
                && f[y as usize][pre] + 1 < f[(y + a) as usize][0]
            {
                f[(y + a) as usize][0] = f[y as usize][pre] + 1;
                q.push_back((y + a, 0));
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_jumps(vec![14, 4, 18, 1, 15], 3, 15, 9));
    }

    #[test]
    fn case2() {
        assert_eq!(
            -1,
            Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::minimum_jumps(vec![1, 6, 2, 14, 5, 17, 4], 16, 9, 7)
        );
    }
}
