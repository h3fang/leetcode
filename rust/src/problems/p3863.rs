pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let s = s.as_bytes();
        if s.is_sorted() {
            return 0;
        }

        let n = s.len();
        if n == 2 {
            return -1;
        }

        let (min, max) = {
            let (mut min, mut max) = (u8::MAX, u8::MIN);
            for &b in s {
                min = min.min(b);
                max = max.max(b);
            }
            (min, max)
        };

        if s[0] == min || s[n - 1] == max {
            return 1;
        }

        if s[1..n - 1].iter().any(|&b| b == max || b == min) {
            return 2;
        }

        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_operations("dog".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_operations("card".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_operations("gf".to_string()));
    }
}
