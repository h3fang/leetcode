pub struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let len = n.len();
        let mut candidates = Vec::with_capacity(5);
        candidates.push(10i64.pow(len as u32) + 1);
        candidates.push(10i64.pow(len as u32 - 1) - 1);
        let prefix = n[..(len + 1) / 2].parse::<i64>().unwrap();
        for i in [prefix, prefix - 1, prefix + 1] {
            let pre = i.to_string();
            let suf = pre.chars().rev().skip(len & 1).collect::<String>();
            let s = pre + &suf;
            candidates.push(s.parse().unwrap());
        }

        let n = n.parse::<i64>().unwrap();
        let mut result = -1;
        for c in candidates {
            if c != n
                && (result == -1
                    || (c - n).abs() < (result - n).abs()
                    || ((c - n).abs() == (result - n).abs() && c < result))
            {
                result = c;
            }
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("121", Solution::nearest_palindromic("123".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("0", Solution::nearest_palindromic("1".to_string()));
    }
}
