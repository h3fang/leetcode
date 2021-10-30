pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();

        let mut result = "";
        for i in 0..2 * s.len() - 1 {
            let j = i / 2;
            if i % 2 == 0 {
                let mut c = 1;
                while j >= c && j + c < s.len() && b[j - c] == b[j + c] {
                    c += 1;
                }
                c -= 1;
                if 1 + 2 * c > result.len() {
                    result = &s[j - c..=j + c];
                }
            } else {
                let mut c = 0;
                while j >= c && j + 1 + c < s.len() && b[j - c] == b[j + 1 + c] {
                    c += 1;
                }
                if 2 * c > result.len() {
                    result = &s[j - c + 1..=j + c];
                }
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "babad".to_string();
        assert!(["bab".to_string(), "aba".to_string()].contains(&Solution::longest_palindrome(s)));
    }

    #[test]
    fn case2() {
        let s = "cbbd".to_string();
        assert_eq!("bb".to_string(), Solution::longest_palindrome(s));
    }
}
