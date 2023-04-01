pub struct Solution;

impl Solution {
    pub fn mask_pii(s: String) -> String {
        fn email(s: String) -> String {
            let (name, domain) = s.split_once('@').unwrap();
            let n = name.as_bytes();
            format!(
                "{}*****{}@{}",
                n[0].to_ascii_lowercase() as char,
                n.last().unwrap().to_ascii_lowercase() as char,
                domain.to_ascii_lowercase()
            )
        }
        fn phone_number(s: String) -> String {
            let digits = s
                .as_bytes()
                .iter()
                .cloned()
                .filter(|&b| b.is_ascii_digit())
                .collect::<Vec<_>>();
            let n = digits.len();
            let digits = unsafe { std::str::from_utf8_unchecked(&digits[n - 4..]) };
            if n == 10 {
                format!("***-***-{}", digits)
            } else {
                format!("+{}-***-***-{}", "*".repeat(n - 10), digits)
            }
        }
        if s.contains('@') {
            email(s)
        } else {
            phone_number(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "l*****e@leetcode.com",
            Solution::mask_pii("LeetCode@LeetCode.com".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "a*****b@qq.com",
            Solution::mask_pii("AB@qq.com".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "***-***-7890",
            Solution::mask_pii("1(234)567-890".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "+**-***-***-5678",
            Solution::mask_pii("86-(10)12345678".to_string())
        );
    }
}
