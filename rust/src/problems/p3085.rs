pub struct Solution;

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut f = [0i32; 26];
        for b in word.bytes() {
            f[(b - b'a') as usize] += 1;
        }
        f.sort_unstable();
        let (mut sum, mut keep, mut r) = (0, 0, 0);
        for &x in &f {
            let max = x + k;
            while r < f.len() && f[r] <= max {
                sum += f[r];
                r += 1;
            }
            keep = keep.max(sum + max * (26 - r as i32));
            sum -= x;
        }
        word.len() as i32 - keep
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_deletions("aabcaba".to_string(), 0));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_deletions("dabdcbdcdcd".to_string(), 2));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::minimum_deletions("aaabaaa".to_string(), 2));
    }
}
