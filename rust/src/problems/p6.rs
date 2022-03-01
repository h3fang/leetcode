pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let n = s.len();
        if num_rows == 1 || num_rows >= n {
            return s;
        }
        let mut result = String::new();
        let t = 2 * num_rows - 2;
        let s = s.as_bytes();
        for i in 0..num_rows {
            for j in (0..n - i).step_by(t) {
                result.push(s[j + i] as char);
                if i > 0 && i < num_rows - 1 && j + t - i < n {
                    result.push(s[j + t - i] as char);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "PAHNAPLSIIGYIR",
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "PINALSIGYAHRPI",
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!("A", Solution::convert("A".to_string(), 1));
    }
}
