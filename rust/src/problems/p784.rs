pub struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn bt(s: &mut [char], i: usize, result: &mut Vec<String>) {
            if i == s.len() {
                result.push(s.iter().collect());
            } else {
                bt(s, i + 1, result);
                let c = s[i];
                if c.is_uppercase() {
                    s[i] = c.to_ascii_lowercase();
                    bt(s, i + 1, result);
                    s[i] = c;
                } else if c.is_lowercase() {
                    s[i] = c.to_ascii_uppercase();
                    bt(s, i + 1, result);
                    s[i] = c;
                }
            }
        }

        let mut s = s.chars().collect::<Vec<_>>();
        let mut result = Vec::new();
        bt(&mut s, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "a1b2".to_string();
        let expected = ["a1b2", "a1B2", "A1b2", "A1B2"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::letter_case_permutation(s);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
