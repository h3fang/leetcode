pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        fn index(c: char) -> usize {
            (c as u8 - b'a') as usize
        }

        if s2.len() < s1.len() {
            return false;
        }

        let mut sig = vec![0i32; 26];
        for c in s1.chars() {
            sig[index(c)] += 1;
        }

        let s2 = s2.chars().collect::<Vec<_>>();

        for c in &s2[..s1.len()] {
            sig[index(*c)] -= 1;
        }

        if sig.iter().all(|c| c == &0) {
            return true;
        }

        for left in 1..s2.len() - s1.len() + 1 {
            sig[index(s2[left - 1])] += 1;
            sig[index(s2[left + s1.len() - 1])] -= 1;
            if sig.iter().all(|c| c == &0) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert_eq!(true, Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn case2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert_eq!(false, Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn case3() {
        let s1 = "ab".to_string();
        let s2 = "ba".to_string();
        assert_eq!(true, Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn case4() {
        let s1 = "adc".to_string();
        let s2 = "dcda".to_string();
        assert_eq!(true, Solution::check_inclusion(s1, s2));
    }
}
