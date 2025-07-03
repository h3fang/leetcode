pub struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        (b'a' + (k - 1).count_ones() as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!('b', Solution::kth_character(5));
    }

    #[test]
    fn case2() {
        assert_eq!('c', Solution::kth_character(10));
    }
}
