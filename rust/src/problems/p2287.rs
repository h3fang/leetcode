pub struct Solution;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut t = [0; 26];
        for b in target.as_bytes() {
            let i = (b - b'a') as usize;
            t[i] += 1;
        }
        let mut c = [0; 26];
        for b in s.as_bytes() {
            let i = (b - b'a') as usize;
            c[i] += 1;
        }
        let mut result = i32::MAX;
        for (&a, &b) in t.iter().zip(&c) {
            if a > 0 {
                result = result.min(b / a);
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
            2,
            Solution::rearrange_characters("ilovecodingonleetcode".into(), "code".into())
        );
    }
}
