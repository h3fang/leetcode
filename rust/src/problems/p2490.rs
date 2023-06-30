pub struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut b = *sentence.as_bytes().last().unwrap();
        sentence.split_whitespace().all(|w| {
            let w = w.as_bytes();
            let prev = b;
            b = *w.last().unwrap();
            w[0] == prev
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_circular_sentence(
            "Leetcode is cool".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(Solution::is_circular_sentence("eetcode".to_string()));
    }
}
