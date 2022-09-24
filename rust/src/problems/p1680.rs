pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        fn len(n: i32) -> u32 {
            32 - n.leading_zeros()
        }
        (1..=n).fold(0, |acc, i| ((acc << len(i)) + i as i64) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::concatenated_binary(1));
    }

    #[test]
    fn case2() {
        assert_eq!(27, Solution::concatenated_binary(3));
    }

    #[test]
    fn case3() {
        assert_eq!(505379714, Solution::concatenated_binary(12));
    }
}
