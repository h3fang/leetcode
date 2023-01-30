pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let (mut t, mut t1, mut t2) = (0, 1, 1);
        for _ in 2..n {
            // destructuring assignments requires rust version >= 1.59
            // (t, t1, t2) = (t1, t2, t + t1 + t2);
            let s = t + t1 + t2;
            t = t1;
            t1 = t2;
            t2 = s;
        }
        t2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::tribonacci(4));
    }

    #[test]
    fn case2() {
        assert_eq!(1389537, Solution::tribonacci(25));
    }
}
