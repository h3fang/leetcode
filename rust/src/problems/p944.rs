pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs[0].len();
        let mut result = 0;
        for c in 0..n {
            let mut prev = 0;
            for b in strs.iter().map(|s| s.as_bytes()[c]) {
                if prev > b {
                    result += 1;
                    break;
                }
                prev = b;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_strs(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::min_deletion_size(parse_strs(&["cba", "daf", "ghi"]))
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_deletion_size(parse_strs(&["a", "b"])));
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::min_deletion_size(parse_strs(&["zyx", "wvu", "tsr"]))
        );
    }
}
