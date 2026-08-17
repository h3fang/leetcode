pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut s = vec![0; n + 1];
        for (i, &x) in stone_value.iter().enumerate() {
            s[i + 1] = s[i] + x;
        }

        let mut f = vec![vec![0; n + 1]; n];
        for i in (0..n - 1).rev() {
            for j in i + 2..n + 1 {
                for k in i + 1..j {
                    let left = s[k] - s[i];
                    let right = s[j] - s[k];
                    let score = match left.cmp(&right) {
                        Ordering::Less => left + f[i][k],
                        Ordering::Equal => (left + f[i][k]).max(right + f[k][j]),
                        Ordering::Greater => right + f[k][j],
                    };
                    f[i][j] = f[i][j].max(score);
                }
            }
        }

        f[0][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(18, Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(28, Solution::stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::stone_game_v(vec![4]));
    }
}
