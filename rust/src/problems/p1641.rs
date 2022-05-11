pub struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = vec![[0; 5]; n as usize + 1];
        dp[1] = [1; 5];
        for i in 2..=n as usize {
            for j in 0..5 {
                dp[i][j] = dp[i - 1][..=j].iter().sum();
            }
        }
        dp[n as usize].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_vowel_strings(1));
    }

    #[test]
    fn case2() {
        assert_eq!(66045, Solution::count_vowel_strings(33));
    }
}
