pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "the sky is blue".to_string();
        let e = "blue is sky the".to_string();
        assert_eq!(e, Solution::reverse_words(s));
    }

    #[test]
    fn case2() {
        let s = "  hello world  ".to_string();
        let e = "world hello".to_string();
        assert_eq!(e, Solution::reverse_words(s));
    }

    #[test]
    fn case3() {
        let s = "a good   example".to_string();
        let e = "example good a".to_string();
        assert_eq!(e, Solution::reverse_words(s));
    }

    #[test]
    fn case4() {
        let s = "  Bob    Loves  Alice   ".to_string();
        let e = "Alice Loves Bob".to_string();
        assert_eq!(e, Solution::reverse_words(s));
    }

    #[test]
    fn case5() {
        let s = "Alice does not even like bob".to_string();
        let e = "bob like even not does Alice".to_string();
        assert_eq!(e, Solution::reverse_words(s));
    }

    #[test]
    fn case6() {
        let s = "a".to_string();
        let e = "a".to_string();
        assert_eq!(e, Solution::reverse_words(s));
    }
}
