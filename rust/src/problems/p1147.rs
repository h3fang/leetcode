pub struct Solution;

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let mut t = text.as_bytes();
        let mut result = 0;
        while !t.is_empty() {
            let n = t.len();
            let mut c = 1;
            while c <= n / 2 && t[..c] != t[n - c..] {
                c += 1;
            }
            if c > n / 2 {
                result += 1;
                break;
            }
            result += 2;
            t = &t[c..n - c];
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
            7,
            Solution::longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_decomposition("merchant".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            11,
            Solution::longest_decomposition("antaprezatepzapreanta".to_string())
        );
    }
}
