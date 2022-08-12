pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim().as_bytes();
        if let Some(first) = s.first() {
            let mut num = 0i64;
            let sign = match first {
                b'+' => 1,
                b'-' => -1,
                f if f.is_ascii_digit() => {
                    num = (f - b'0') as i64;
                    1
                }
                _ => return 0,
            };
            for b in &s[1..] {
                if !b.is_ascii_digit() {
                    break;
                }
                num = num * 10 + (b - b'0') as i64;
                if sign == -1 && num > -(i32::MIN as i64) {
                    return i32::MIN;
                }
                if sign == 1 && num > i32::MAX as i64 {
                    return i32::MAX;
                }
            }
            (num * sign) as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(42, Solution::my_atoi("42".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(-42, Solution::my_atoi("   -42".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(4193, Solution::my_atoi("4193 with words".to_string()));
    }
}
