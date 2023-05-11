pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        fn check(s: &[u8], k: i32, lb: i32, ub: i32) -> bool {
            let mut seen = HashSet::new();
            let mut t = 0;
            for (i, &r) in s.iter().enumerate() {
                t = t * 2 + (r - b'0') as i32;
                if i as i32 >= k {
                    t -= ((s[i - k as usize] - b'0') as i32) << k;
                }
                if i as i32 > k - 1 && t >= lb && t <= ub {
                    seen.insert(t);
                }
            }
            seen.len() as i32 == ub - lb + 1
        }
        let s = s.as_bytes();
        let m = s.len() as i32;
        if m == 1 {
            return s.contains(&b'1');
        }
        let mut k = 30;
        while (1 << k) > n {
            k -= 1;
        }
        let b1 = (1 << (k - 1)) + k - 1;
        let b2 = n - (1 << k) + k + 1;
        if m < b1 || m < b2 {
            return false;
        }
        check(s, k, 1 << (k - 1), (1 << k) - 1) && check(s, k + 1, 1 << k, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::query_string("0110".to_string(), 3));
    }

    #[test]
    fn case2() {
        assert!(!Solution::query_string("0110".to_string(), 4));
    }
}
