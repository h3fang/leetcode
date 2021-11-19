pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::hamming_distance(1, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::hamming_distance(3, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(30, Solution::hamming_distance(1, i32::MAX));
    }
}
