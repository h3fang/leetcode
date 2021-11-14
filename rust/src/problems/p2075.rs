pub struct Solution;

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let n = encoded_text.len();
        let cols = n / rows as usize;
        let mut result = String::new();
        let encoded_text = encoded_text.as_bytes();
        for i in 0..cols {
            for j in (i..n).step_by(cols + 1) {
                result.push(encoded_text[j] as char);
            }
        }
        result.trim_end().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let encoded_text = "ch   ie   pr".to_string();
        let rows = 3;
        assert_eq!(
            "cipher".to_string(),
            Solution::decode_ciphertext(encoded_text, rows)
        );
    }

    #[test]
    fn case2() {
        let encoded_text = "iveo    eed   l te   olc".to_string();
        let rows = 4;
        assert_eq!(
            "i love leetcode".to_string(),
            Solution::decode_ciphertext(encoded_text, rows)
        );
    }
}
