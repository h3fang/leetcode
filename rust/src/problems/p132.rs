pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut cut = vec![0; n];

        let s = s.as_bytes();
        for j in 0..n {
            cut[j] = j;
            for i in 0..=j {
                if s[i] == s[j] && ((j - i <= 1) || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    cut[j] = if i == 0 {
                        0
                    } else {
                        cut[j].min(cut[i - 1] + 1)
                    };
                }
            }
        }

        cut[n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_cut("aab".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_cut("aa".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::min_cut("ab".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::min_cut("efe".to_string()));
    }
}
