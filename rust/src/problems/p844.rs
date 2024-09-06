pub struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn type_into_editor(s: String) -> String {
            s.chars().fold("".to_string(), |mut acc, c| {
                if c == '#' {
                    acc.pop();
                } else {
                    acc.push(c);
                }
                acc
            })
        }

        type_into_editor(s) == type_into_editor(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(Solution::backspace_compare(
            "a##c".to_string(),
            "#a#c".to_string()
        ));
    }

    #[test]
    fn case4() {
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }
}
