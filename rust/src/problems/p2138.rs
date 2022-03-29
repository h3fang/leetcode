pub struct Solution;

impl Solution {
    pub fn divide_string(mut s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let r = s.len() % k;
        if r > 0 {
            for _ in 0..k - r {
                s.push(fill);
            }
        }
        s.as_bytes()
            .chunks(k)
            .map(|g| unsafe { std::str::from_utf8_unchecked(g).to_string() })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string(),],
            Solution::divide_string("abcdefghi".to_string(), 3, 'x')
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jxx".to_string(),
            ],
            Solution::divide_string("abcdefghij".to_string(), 3, 'x')
        );
    }
}
