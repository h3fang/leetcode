pub struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let (mut lower, mut upper) = (0u32, 0u32);
        for b in word.bytes() {
            if b.is_ascii_uppercase() {
                upper |= 1 << (b - b'A');
            } else {
                lower |= 1 << (b - b'a');
            }
        }
        (lower & upper).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::number_of_special_chars("aaAbcBC".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::number_of_special_chars("abc".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::number_of_special_chars("abBCab".to_string()));
    }
}
