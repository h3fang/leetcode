pub struct Solution;

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let e = (n & 0b10101010101).count_ones() as i32;
        let o = (n & 0b01010101010).count_ones() as i32;
        vec![e, o]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 0], Solution::even_odd_bit(17));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0, 1], Solution::even_odd_bit(2));
    }
}
