pub struct Solution;

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        let m = first.len();
        let n = second.len();
        if m == n {
            return first
                .chars()
                .zip(second.chars())
                .filter(|(a, b)| a == b)
                .count()
                <= 1;
        }
        let (a, b) = if m <= n {
            (first.as_bytes(), second.as_bytes())
        } else {
            (second.as_bytes(), first.as_bytes())
        };
        if (b.len() - a.len()) > 1 {
            return false;
        }
        for i in 0..a.len() {
            if a[i] != b[i] {
                return a[i..] == b[i + 1..];
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::one_edit_away("pale".into(), "ple".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::one_edit_away("pales".into(), "pal".into()));
    }
}
