pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let (mut f0, mut f1) = (3, 12);
        for _ in 2..n + 1 {
            (f1, f0) = ((f1 * 5 - f0 * 2) % MOD, f1);
        }
        ((f1 + MOD) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::num_of_ways(1));
    }

    #[test]
    fn case2() {
        assert_eq!(54, Solution::num_of_ways(2));
    }

    #[test]
    fn case3() {
        assert_eq!(246, Solution::num_of_ways(3));
    }

    #[test]
    fn case4() {
        assert_eq!(106494, Solution::num_of_ways(7));
    }

    #[test]
    fn case5() {
        assert_eq!(30228214, Solution::num_of_ways(5000));
    }
}
