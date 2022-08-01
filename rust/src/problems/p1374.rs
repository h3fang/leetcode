pub struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let n = n as usize;
        if n % 2 == 1 {
            "a".repeat(n)
        } else {
            "a".repeat(n - 1) + "b"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid(s: &str, n: i32) {
        assert_eq!(n as usize, s.len());
        let mut freq = [0; 26];
        for c in s.chars() {
            assert!(c.is_ascii_lowercase());
            let i = (c as u8 - b'a') as usize;
            freq[i] += 1;
        }
        assert!(freq.iter().all(|&f| f == 0 || f % 2 == 1));
    }

    #[test]
    fn case1() {
        let n = 4;
        let result = Solution::generate_the_string(n);
        assert_valid(&result, n);
    }

    #[test]
    fn case2() {
        let n = 27;
        let result = Solution::generate_the_string(n);
        assert_valid(&result, n);
    }
}
