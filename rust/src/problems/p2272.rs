pub struct Solution;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        if n < 3 {
            return 0;
        }
        let mut result = 0;
        for a in b'a'..=b'z' {
            for b in b'a'..=b'z' {
                if a == b {
                    continue;
                }
                let mut dp0 = i32::MIN / 2;
                let mut dp1 = i32::MIN / 2;
                for &c in s {
                    let v = if c == a {
                        1
                    } else if c == b {
                        -1
                    } else {
                        0
                    };
                    if c == b {
                        dp1 = (dp0 + v).max(dp1 + v).max(v);
                    } else {
                        dp1 += v;
                    }
                    dp0 = (dp0 + v).max(v);
                    result = result.max(dp1);
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
        assert_eq!(3, Solution::largest_variance("aababbb".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::largest_variance("abcde".into()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::largest_variance("bba".into()));
    }
}
