pub struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len() / 2;
        let count = |s: &[u8]| {
            s.iter()
                .filter(|&b| [b'a', b'e', b'i', b'o', b'u'].contains(&b.to_ascii_lowercase()))
                .count()
        };
        count(&s[..n]) == count(&s[n..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::halves_are_alike("book".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::halves_are_alike("textbook".to_string()));
    }
}
