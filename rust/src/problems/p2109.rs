pub struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::with_capacity(s.len() + spaces.len());
        let mut i = 0;
        for c in s.chars() {
            while i < spaces.len() && spaces[i] as usize + i == result.len() {
                i += 1;
                result.push(' ');
            }
            result.push(c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "LeetcodeHelpsMeLearn".to_string();
        let spaces = vec![8, 13, 15];
        assert_eq!("Leetcode Helps Me Learn", Solution::add_spaces(s, spaces));
    }
}
