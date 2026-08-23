pub struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let n = num.len() / 2;
        let mut d = 0;
        for (i, &b) in num.as_bytes().iter().enumerate() {
            let v = if b == b'?' {
                9
            } else {
                2 * i32::from(b - b'0')
            };
            d += if i < n { v } else { -v };
        }
        d != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::sum_game("5023".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::sum_game("25??".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::sum_game("5023".to_string()));
    }
}
