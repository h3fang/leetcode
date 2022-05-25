pub struct Solution;

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let p = p.as_bytes();
        let mut dp = [0; 26];
        dp[(p[0] - b'a') as usize] = 1;
        let mut k = 1;
        for w in p.windows(2) {
            if (w[0] == b'z' && w[1] == b'a') || (w[1] == w[0] + 1) {
                k += 1;
            } else {
                k = 1;
            }
            let i = (w[1] - b'a') as usize;
            dp[i] = dp[i].max(k);
        }
        dp.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::find_substring_in_wrapround_string("a".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::find_substring_in_wrapround_string("cac".into())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            6,
            Solution::find_substring_in_wrapround_string("zab".into())
        );
    }
}
