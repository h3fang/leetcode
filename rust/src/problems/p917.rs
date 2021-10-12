pub struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut left = 0;
        let mut right = chars.len() - 1;
        'first: while left < right {
            while !chars[left].is_alphabetic() {
                left += 1;

                if left >= right {
                    break 'first;
                }
            }
            while !chars[right].is_alphabetic() {
                right -= 1;

                if left >= right {
                    break 'first;
                }
            }

            chars.swap(left, right);
            left += 1;
            right -= 1;
        }
        chars.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "7_28]".to_string(),
            Solution::reverse_only_letters("7_28]".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "j-Ih-gfE-dCba".to_string(),
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string())
        );
    }
}
