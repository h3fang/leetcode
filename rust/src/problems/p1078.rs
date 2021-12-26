pub struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words = text.split_ascii_whitespace().collect::<Vec<_>>();
        let mut result = vec![];
        for w in words.windows(3) {
            if w[0] == first && w[1] == second {
                result.push(w[2].to_string());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let text = "alice is a good girl she is a good student".to_string();
        let first = "a".to_string();
        let second = "good".to_string();
        assert_eq!(
            vec!["girl".to_string(), "student".to_string()],
            Solution::find_ocurrences(text, first, second)
        );
    }
}
