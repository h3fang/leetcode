pub struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let s = s.as_bytes();
        let (n, mut ans, mut cnt) = (s.len(), s.len(), 0);
        for i in 0..n * 2 - 1 {
            cnt += (s[i % n] as usize ^ i) & 1;
            if i + 1 < n {
                continue;
            }
            let left = i + 1 - n;
            ans = ans.min(cnt).min(n - cnt);
            cnt -= (s[left] as usize ^ left) & 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_flips("111000".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_flips("010".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::min_flips("1110".to_string()));
    }
}
