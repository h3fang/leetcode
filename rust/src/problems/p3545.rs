pub struct Solution;

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut f = [0; 26];
        for b in s.bytes() {
            f[(b - b'a') as usize] += 1;
        }
        f.sort_unstable();
        f[..f.len() - k as usize].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_deletion("abc".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_deletion("aabb".to_string(), 2));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::min_deletion("yyyzz".to_string(), 1));
    }
}
