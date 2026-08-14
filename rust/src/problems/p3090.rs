pub struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut f = [0; 26];
        let (mut l, mut ans) = (0, 0);
        for (r, b) in s.iter().enumerate() {
            let i = (b - b'a') as usize;
            f[i] += 1;
            while f[i] > 2 {
                f[(s[l] - b'a') as usize] -= 1;
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::maximum_length_substring("bcbbbcba".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::maximum_length_substring("aaaa".to_string()));
    }
}
