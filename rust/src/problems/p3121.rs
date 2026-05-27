pub struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower = [usize::MAX; 26];
        for (i, b) in word.bytes().enumerate() {
            if b.is_ascii_lowercase() {
                let j = (b - b'a') as usize;
                lower[j] = i;
            }
        }

        let mut upper = [usize::MAX; 26];
        for (i, b) in word.bytes().enumerate().rev() {
            if b.is_ascii_uppercase() {
                let j = (b - b'A') as usize;
                upper[j] = i;
            }
        }

        let mut ans = 0;
        for (&l, &u) in lower.iter().zip(&upper) {
            if u != usize::MAX && l < u {
                ans += 1;
            }
        }
        ans
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
        assert_eq!(0, Solution::number_of_special_chars("AbBCab".to_string()));
    }
}
