pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut f = [0; 3];
        let (mut l, mut ans) = (0, 0);
        let s = s.as_bytes();
        for &b in s {
            f[(b - b'a') as usize] += 1;
            while f.iter().all(|&x| x > 0) {
                f[(s[l] - b'a') as usize] -= 1;
                l += 1;
            }
            ans += l as i32;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::number_of_substrings("abcabc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::number_of_substrings("aaacb".to_string()));
    }
}
