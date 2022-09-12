pub struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut result = 1;
        let mut mask = 0u32;
        for &b in s.as_bytes() {
            let i = (b - b'a') as usize;
            if mask & (1 << i) > 0 {
                result += 1;
                mask = 1 << i;
            } else {
                mask |= 1 << i;
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
        assert_eq!(4, Solution::partition_string("abacaba".to_string()));
    }
}
