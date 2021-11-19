pub struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n as u32;
        let mut r = 0;
        while n > 1 {
            if n & 1 > 0 {
                if n == 3 {
                    return r + 2;
                } else if n & 3 == 3 {
                    n += 1;
                }
                r += 2;
            } else {
                r += 1;
            }
            n >>= 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::integer_replacement(8));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::integer_replacement(7));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::integer_replacement(4));
    }

    #[test]
    fn case4() {
        assert_eq!(17, Solution::integer_replacement(65535));
    }

    #[test]
    fn case5() {
        assert_eq!(2, Solution::integer_replacement(3));
    }

    #[test]
    fn case6() {
        assert_eq!(32, Solution::integer_replacement(2147483647));
    }
}
