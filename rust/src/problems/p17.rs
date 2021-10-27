pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn bt(digits: &str, table: &[Vec<char>], curr: &mut String, result: &mut Vec<String>) {
            match digits.chars().next() {
                None => result.push(curr.to_owned()),
                Some(d) => {
                    for c in &table[(d as u8 - b'2') as usize] {
                        curr.push(*c);
                        bt(&digits[1..], table, curr, result);
                        curr.pop();
                    }
                }
            }
        }

        let mut result = Vec::new();
        if digits.is_empty() {
            return result;
        }
        let table = [
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut curr = String::new();
        bt(&digits, &table, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::letter_combinations("23".to_string());
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
