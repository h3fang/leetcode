pub struct Solution;

fn freq(s: &[u8]) -> [u32; 26] {
    let mut f = [0; 26];
    for b in s.iter().step_by(2) {
        f[(b - b'a') as usize] += 1;
    }
    f
}

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        freq(s1) == freq(s2) && freq(&s1[1..]) == freq(&s2[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_strings(
            "abcdba".to_string(),
            "cabdab".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_strings(
            "abe".to_string(),
            "bea".to_string()
        ));
    }
}
