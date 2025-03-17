pub struct Solution;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        if n < 3 {
            return 0;
        }
        let mut m = vec![vec![]; 26];
        for (i, &b) in s.iter().enumerate() {
            m[(b - b'a') as usize].push(i);
        }
        let mut result = 0;
        for (i, a) in m.iter().enumerate() {
            if a.is_empty() {
                continue;
            }
            for (j, b) in m.iter().enumerate() {
                if i == j || b.is_empty() {
                    continue;
                }
                let (mut i, mut j) = (0, 0);
                let (mut f0, mut f1) = (0, i32::MIN / 2);
                while i < a.len() || j < b.len() {
                    if j == b.len() || (i < a.len() && a[i] < b[j]) {
                        f0 = (f0 + 1).max(1);
                        f1 += 1;
                        i += 1;
                    } else {
                        f1 = (f0 - 1).max(f1 - 1).max(-1);
                        f0 = (f0 - 1).max(-1);
                        j += 1;
                    }
                    result = result.max(f1);
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
        assert_eq!(3, Solution::largest_variance("aababbb".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::largest_variance("abcde".into()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::largest_variance("bba".into()));
    }
}
