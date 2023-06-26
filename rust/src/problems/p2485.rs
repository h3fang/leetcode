pub struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total = (1 + n) * n / 2;
        let mut p1 = 0;
        for i in 1..=n {
            let p2 = total - p1;
            p1 += i;
            if p1 == p2 {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::pivot_integer(8));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::pivot_integer(4));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::pivot_integer(1));
    }
}
