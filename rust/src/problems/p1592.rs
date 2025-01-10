pub struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let spaces = text.as_bytes().iter().filter(|b| **b == b' ').count();
        let words = text.split_ascii_whitespace().count();
        let n = if words > 1 { spaces / (words - 1) } else { 0 };
        let mut result = String::with_capacity(text.len());
        for w in text.split_ascii_whitespace() {
            if !result.is_empty() {
                result.push_str(&" ".repeat(n));
            }
            result.push_str(w);
        }
        result.push_str(&" ".repeat(spaces - (words - 1) * n));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "this   is   a   sentence",
            Solution::reorder_spaces("  this   is  a sentence ".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "practice   makes   perfect ",
            Solution::reorder_spaces(" practice   makes   perfect".to_string())
        );
    }
}
