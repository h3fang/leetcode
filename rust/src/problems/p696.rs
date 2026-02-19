pub struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let (mut pre, mut cur) = (0, 0);
        let (mut last, mut ans) = (b'#', 0);
        for b in s.bytes().chain([b'#']) {
            if b != last {
                ans += pre.min(cur);
                (pre, cur) = (cur, 1);
            } else {
                cur += 1;
            }
            last = b;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::count_binary_substrings("00110011".to_string()));
    }
    #[test]
    fn case2() {
        assert_eq!(4, Solution::count_binary_substrings("10101".to_string()));
    }
}
