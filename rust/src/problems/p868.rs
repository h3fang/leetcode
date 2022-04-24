pub struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut result = 0;
        let mut last = -1;
        let mut i = 0;
        while n > 0 {
            if n & 1 > 0 {
                if last >= 0 {
                    result = result.max(i - last);
                }
                last = i;
            }
            i += 1;
            n >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::binary_gap(22));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::binary_gap(8));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::binary_gap(5));
    }
}
