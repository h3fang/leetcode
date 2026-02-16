pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: i32) -> i32 {
        x.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(964176192, Solution::reverse_bits(43261596));
    }

    #[test]
    fn case2() {
        assert_eq!(1073741822, Solution::reverse_bits(2147483644));
    }
}
