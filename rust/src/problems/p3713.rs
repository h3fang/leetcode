pub struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;

        let mut f = [0; 26];
        for i in 0..s.len() {
            f.fill(0);
            let (mut cnt, mut max) = (0, 0);
            for (j, x) in s.iter().enumerate().skip(i) {
                let k = (x - b'a') as usize;
                if f[k] == 0 {
                    cnt += 1;
                }
                f[k] += 1;
                max = max.max(f[k]);
                if max * cnt == j - i + 1 {
                    ans = ans.max(j - i + 1);
                }
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::longest_balanced("abbac".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::longest_balanced("zzabccy".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::longest_balanced("aba".to_string()));
    }
}
