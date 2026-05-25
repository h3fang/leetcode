pub struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let (min, max) = (min_jump as usize, max_jump as usize);
        let s = s.as_bytes();
        let n = s.len();
        let mut d = vec![0; n + 1];
        d[0] = 1;
        d[1] = -1;
        let mut sum = 0;
        for (i, &b) in s.iter().enumerate() {
            sum += d[i];
            if sum > 0 && b == b'0' {
                d[n.min(i + min)] += 1;
                d[n.min(i + max + 1)] -= 1;
            }
        }
        sum > 0 && s[n - 1] == b'0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_reach("011010".to_string(), 2, 3));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_reach("01101110".to_string(), 2, 3));
    }
}
