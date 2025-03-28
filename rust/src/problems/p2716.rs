pub struct Solution;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        s.bytes()
            .fold(0u32, |acc, b| acc | (1 << (b - b'a')))
            .count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimized_string_length("aaabc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimized_string_length("cbbd".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::minimized_string_length("dddaaa".to_string()));
    }
}
