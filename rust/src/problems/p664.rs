pub struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut f = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            f[i][i] = 1;
            for j in i + 1..n {
                if s[i] == s[j] {
                    f[i][j] = f[i][j - 1];
                } else {
                    let mut min = i32::MAX;
                    for k in i..j {
                        min = min.min(f[i][k] + f[k + 1][j]);
                    }
                    f[i][j] = min;
                }
            }
        }
        f[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::strange_printer("aaabbb".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::strange_printer("aba".to_string()));
    }
}
