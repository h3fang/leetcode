pub struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp0 = 0;
        let mut dp1 = 0;
        for &b in s {
            let d0 = dp0 + if b == b'1' { 1 } else { 0 };
            let d1 = dp0.min(dp1 + if b == b'1' { 0 } else { 1 });
            dp0 = d0;
            dp1 = d1;
        }
        dp0.min(dp1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_flips_mono_incr("00110".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_flips_mono_incr("010110".into()));
    }
}
