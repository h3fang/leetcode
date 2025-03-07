pub struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut title = title
            .split_ascii_whitespace()
            .map(|w| {
                let mut w = w.to_ascii_lowercase();
                if w.len() > 2 {
                    unsafe {
                        w.as_bytes_mut()[0] = w.as_bytes_mut()[0].to_ascii_uppercase();
                    }
                }
                w.push(' ');
                w
            })
            .collect::<String>();
        title.pop();
        title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "Capitalize The Title",
            Solution::capitalize_title("capiTalIze tHe titLe".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "First Letter of Each Word",
            Solution::capitalize_title("First leTTeR of EACH Word".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "i Love Leetcode",
            Solution::capitalize_title("i lOve leetcode".to_string())
        );
    }
}
