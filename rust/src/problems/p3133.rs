pub struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let bits = 64 - n.leading_zeros() - x.leading_zeros();
        let mut result = x as i64;
        let n = n as i64 - 1;
        let mut j = 0;
        for i in 0..bits {
            if (result >> i) & 1 == 0 {
                if ((n >> j) & 1) != 0 {
                    result |= 1 << i;
                }
                j += 1;
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
        assert_eq!(6, Solution::min_end(3, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::min_end(2, 7));
    }
}
