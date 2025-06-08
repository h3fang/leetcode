pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let min = complexity[0];
        let mut ans = 1i64;
        for (i, &c) in complexity.iter().enumerate().skip(1) {
            if min >= c {
                return 0;
            }
            ans = (ans * i as i64) % MOD;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_permutations(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_permutations(vec![3, 3, 3, 4, 4, 4]));
    }
}
