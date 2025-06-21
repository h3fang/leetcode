pub struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        s.as_bytes()
            .chunks(k)
            .map(|g| {
                let mut s = g.to_vec();
                s.resize(k, fill as u8);
                unsafe { String::from_utf8_unchecked(s) }
            })
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
