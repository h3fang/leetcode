pub struct Solution;

impl Solution {
    pub fn num_of_subsequences(s: String) -> i64 {
        let n = s.len();
        let mut pre = vec![0; n + 1];
        for (i, &b) in s.as_bytes().iter().enumerate() {
            pre[i + 1] = pre[i] + if b == b'L' { 1 } else { 0 };
        }
        let mut suf = vec![0; n + 1];
        for (i, &b) in s.as_bytes().iter().enumerate().rev() {
            suf[i] = suf[i + 1] + if b == b'T' { 1 } else { 0 };
        }
        let mut ans = [0; 3];
        let mut max = 0;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            if b == b'C' {
                ans[0] += (pre[i] + 1) * suf[i];
                ans[1] += pre[i] * suf[i];
                ans[2] += pre[i] * (suf[i] + 1);
            }
            max = max.max(pre[i] * suf[i]);
        }
        ans[1] += max;
        ans.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::num_of_subsequences("LMCT".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::num_of_subsequences("LCCT".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::num_of_subsequences("L".to_string()));
    }
}
