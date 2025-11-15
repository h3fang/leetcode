pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut ans, mut ones) = (0, 0);
        let mut zeros = vec![-1];
        for (r, &b) in s.iter().enumerate() {
            if b == b'0' {
                zeros.push(r as i32);
            } else {
                ones += 1;
                ans += r as i32 - zeros.last().unwrap();
            }
            for (i, &z) in zeros.iter().enumerate().skip(1).rev() {
                let n_zero = (zeros.len() - i) as i32;
                if n_zero * n_zero > ones {
                    break;
                }
                let ones = r as i32 + 1 - z - n_zero;
                ans += (z - (n_zero * n_zero - ones).max(0) - zeros[i - 1]).max(0);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::number_of_substrings("00011".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::number_of_substrings("101101".to_string()));
    }
}
