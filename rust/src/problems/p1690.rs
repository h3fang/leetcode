pub struct Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut s = vec![0; n + 1];
        for (i, &x) in stones.iter().enumerate() {
            s[i + 1] = s[i] + x;
        }
        let mut f = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                f[i][j] = (s[j + 1] - s[i + 1] - f[i + 1][j]).max(s[j] - s[i] - f[i][j - 1]);
            }
        }
        f[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::stone_game_vii(vec![5, 3, 1, 4, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            122,
            Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2])
        );
    }
}
