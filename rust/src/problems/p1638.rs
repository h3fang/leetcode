pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let m = s.len();
        let n = t.len();
        let mut dpl = vec![vec![0; n + 1]; m + 1];
        let mut dpr = vec![vec![0; n + 1]; m + 1];
        for (i, &a) in s.iter().enumerate() {
            for (j, &b) in t.iter().enumerate() {
                dpl[i + 1][j + 1] = if a == b { dpl[i][j] + 1 } else { 0 };
            }
        }
        for (i, &a) in s.iter().enumerate().rev() {
            for (j, &b) in t.iter().enumerate().rev() {
                dpr[i][j] = if a == b { dpr[i + 1][j + 1] + 1 } else { 0 };
            }
        }
        let mut result = 0;
        for (i, &a) in s.iter().enumerate() {
            for (j, &b) in t.iter().enumerate() {
                if a != b {
                    result += (dpl[i][j] + 1) * (dpr[i + 1][j + 1] + 1);
                }
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
        assert_eq!(
            6,
            Solution::count_substrings("aba".to_string(), "baba".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::count_substrings("ab".to_string(), "bb".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            10,
            Solution::count_substrings("abe".to_string(), "bbc".to_string())
        );
    }
}
