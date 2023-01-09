pub struct Solution;

impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        let mut step = 1;
        let mut pow = 2;
        while pow != 1 {
            step += 1;
            pow = (pow * 2) % (n - 1);
        }
        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::reinitialize_permutation(2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::reinitialize_permutation(4));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::reinitialize_permutation(6));
    }
}
