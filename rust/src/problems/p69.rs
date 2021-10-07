pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut a = 0;
        let mut b = if x > 1 { x / 2 } else { x };
        let mut result = 0;
        while a <= b {
            let m = a + (b - a) / 2;
            let square = m as u64 * m as u64;
            if square > x as u64 {
                b = m - 1;
            } else if square <= x as u64 {
                a = m + 1;
                result = m;
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
        assert_eq!(2, Solution::my_sqrt(4));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::my_sqrt(8));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::my_sqrt(1));
    }

    #[test]
    fn case4() {
        assert_eq!(46340, Solution::my_sqrt(2147395600));
    }
}
