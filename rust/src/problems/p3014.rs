pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len() as i32;
        let k = n / 8;
        (4 * k + n % 8) * (k + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_string()));
    }

    #[test]
    fn case12() {
        assert_eq!(12, Solution::minimum_pushes("xycdefghij".to_string()));
    }
}
