pub struct Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let n = s
            .as_bytes()
            .iter()
            .map(|b| {
                let b = b - b'a' + 1;
                (b / 10 + b % 10) as i32
            })
            .sum::<i32>();
        (0..k - 1).fold(n, |mut n, _| {
            let mut m = 0;
            while n > 0 {
                m += n % 10;
                n /= 10;
            }
            m
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(36, Solution::get_lucky("iiii".to_string(), 1));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::get_lucky("leetcode".to_string(), 2));
    }
}
