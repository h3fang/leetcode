pub struct Solution;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        fn index(b: u8) -> usize {
            match b {
                b'Q' => 0,
                b'W' => 1,
                b'E' => 2,
                b'R' => 3,
                _ => unreachable!(),
            }
        }
        fn check(f: &[i32; 4], p: i32) -> bool {
            f.iter().all(|&c| c <= p)
        }
        let s = s.as_bytes();
        let n = s.len();
        let mut f = [0; 4];
        let p = (n / 4) as i32;
        for &b in s {
            f[index(b)] += 1;
        }
        if check(&f, p) {
            return 0;
        }
        let mut result = n;
        let mut r = 0;
        for (l, &b) in s.iter().enumerate() {
            while r < n && !check(&f, p) {
                f[index(s[r])] -= 1;
                r += 1;
            }
            if !check(&f, p) {
                break;
            }
            result = result.min(r - l);
            f[index(b)] += 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::balanced_string("QWER".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::balanced_string("QQER".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::balanced_string("QQQW".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(3, Solution::balanced_string("QQQQ".to_string()));
    }
}
