pub struct Solution;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut f = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                f[i][j] = i32::from(s[i] != s[j]) + if j - i <= 2 { 0 } else { f[i + 1][j - 1] };
            }
        }
        let k = k as usize;
        let mut g: Vec<i32> = (0..n).map(|i| f[0][i]).collect();
        for i in 1..k {
            for j in (i..=n - k + i).rev() {
                for p in i..=j {
                    g[j] = g[j].min(f[p][j] + g[p - 1]);
                }
            }
        }
        g[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::palindrome_partition("abc".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::palindrome_partition("aabbc".to_string(), 3));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::palindrome_partition("leetcode".to_string(), 8));
    }
}
