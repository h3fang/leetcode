pub struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        let mut depth = 0;
        for b in s {
            match b {
                b'(' => {
                    depth += 1;
                    result = result.max(depth);
                }
                b')' => depth -= 1,
                _ => {}
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
        assert_eq!(3, Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()));
    }
}
