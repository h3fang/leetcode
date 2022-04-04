pub struct Solution;

impl Solution {
    pub fn sum_scores(s: String) -> i64 {
        fn z_function(s: &[u8]) -> Vec<usize> {
            let n = s.len();
            let mut z = vec![0; n];
            z[0] = n;
            let mut l = 0;
            let mut r = 0;
            for i in 1..n {
                if i <= r && z[i - l] < r - i + 1 {
                    z[i] = z[i - l];
                } else {
                    z[i] = if r + 1 < i { 0 } else { r + 1 - i };
                    while i + z[i] < n && s[z[i] as usize] == s[i + z[i] as usize] {
                        z[i] += 1;
                    }
                }
                if i + z[i] - 1 > r {
                    l = i;
                    r = i + z[i] - 1;
                }
            }
            z
        }
        let z = z_function(s.as_bytes());
        z.iter().sum::<usize>() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::sum_scores("babab".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(14, Solution::sum_scores("azbazbzaz".to_string()));
    }
}
