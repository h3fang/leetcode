pub struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        fn gcd(n1: usize, n2: usize) -> usize {
            if n2 == 0 {
                return n1;
            }
            gcd(n2, n1 % n2)
        }
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return "".to_string();
        }
        let length = gcd(str1.len(), str2.len());
        str1[0..length].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "ABC",
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "AB",
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "",
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string())
        );
    }
}
