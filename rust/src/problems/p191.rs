pub struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::hammingWeight(0b00000000000000000000000000001011)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::hammingWeight(0b00000000000000000000000010000000)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            31,
            Solution::hammingWeight(0b11111111111111111111111111111101)
        );
    }
}
