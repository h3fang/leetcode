pub struct Solution;

impl Solution {
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        if let Some(i) = word.find(ch) {
            let w = unsafe { word.as_mut_vec() };
            w[..=i].reverse();
        }
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "dcbaefd",
            Solution::reverse_prefix("abcdefd".to_string(), 'd')
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "zxyxxe",
            Solution::reverse_prefix("xyxzxe".to_string(), 'z')
        );
    }

    #[test]
    fn case3() {
        assert_eq!("abc", Solution::reverse_prefix("abc".to_string(), 'z'));
    }
}
