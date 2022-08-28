pub struct Solution;

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut count = [0; 10];
        for n in num.as_bytes() {
            let i = (n - b'0') as usize;
            count[i] += 1;
        }
        let mut result = String::with_capacity(num.len());
        for (i, &d) in count.iter().enumerate().rev() {
            if d >= 2 {
                let c = (i as u8 + b'0') as char;
                let s = c.to_string().repeat(d / 2);
                result.push_str(&s);
            }
        }
        if let Some(d) = (0..=9).filter(|d| count[*d] % 2 == 1).max() {
            let c = (d as u8 + b'0') as char;
            result.push(c);
        }
        for (i, &d) in count.iter().enumerate() {
            if d >= 2 {
                let c = (i as u8 + b'0') as char;
                let s = c.to_string().repeat(d / 2);
                result.push_str(&s);
            }
        }
        let result = result.trim_start_matches('0').trim_end_matches('0');
        if result.is_empty() {
            "0".to_string()
        } else {
            result.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "7449447",
            Solution::largest_palindromic("444947137".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!("9", Solution::largest_palindromic("00009".to_string()));
    }
}
