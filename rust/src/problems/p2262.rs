pub struct Solution;

impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let mut pre = [-1; 26];
        let mut sum = 0;
        let mut result = 0;
        for (i, c) in s.char_indices() {
            let c = (c as u8 - b'a') as usize;
            sum += i as i32 - pre[c];
            result += sum as i64;
            pre[c] = i as i32;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(28, Solution::appeal_sum("abbca".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(20, Solution::appeal_sum("code".into()));
    }
}
