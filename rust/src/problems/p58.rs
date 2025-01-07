pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::length_of_last_word("Hello World".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            6,
            Solution::length_of_last_word("luffy is still joyboy".to_string())
        );
    }
}
