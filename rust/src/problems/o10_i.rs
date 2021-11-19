pub struct Solution;

const MOD: i32 = 1000000007;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            0
        } else {
            (1..n)
                .fold((0, 1), |acc, _| (acc.1, (acc.0 + acc.1) % MOD))
                .1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::fib(2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::fib(0));
    }

    #[test]
    fn case3() {
        assert_eq!(832040, Solution::fib(30));
    }
}
