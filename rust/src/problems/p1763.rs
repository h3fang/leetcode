pub struct Solution;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let n = s.len();
        let mut max_pos = 0;
        let mut max_len = 0;
        for i in 0..n {
            let mut lower = 0;
            let mut upper = 0;
            for (j, c) in s.chars().enumerate().skip(i) {
                if c.is_lowercase() {
                    lower |= 1 << (c as i32 - 'a' as i32);
                } else {
                    upper |= 1 << (c as i32 - 'A' as i32);
                }
                if lower == upper && j - i + 1 > max_len {
                    max_pos = i;
                    max_len = j - i + 1;
                }
            }
        }
        s[max_pos..max_pos + max_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "aAa",
            Solution::longest_nice_substring("YazaAay".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!("Bb", Solution::longest_nice_substring("Bb".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!("", Solution::longest_nice_substring("c".to_string()));
    }
}
