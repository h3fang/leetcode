pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs[0].len();
        let mut f = vec![0; m];
        for i in 0..m {
            for j in 0..i {
                if f[j] > f[i] && strs.iter().all(|s| s.as_bytes()[j] <= s.as_bytes()[i]) {
                    f[i] = f[j];
                }
            }
            f[i] += 1;
        }
        (m - f.iter().max().unwrap()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = ["babca", "bbazb"].iter().map(|s| s.to_string()).collect();
        assert_eq!(3, Solution::min_deletion_size(strs));
    }

    #[test]
    fn case4() {
        let strs = ["edcba"].iter().map(|s| s.to_string()).collect();
        assert_eq!(4, Solution::min_deletion_size(strs));
    }

    #[test]
    fn case3() {
        let strs = ["ghi", "def", "abc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(0, Solution::min_deletion_size(strs));
    }
}
