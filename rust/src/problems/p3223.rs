pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut f = [0; 26];
        for b in s.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        f.into_iter()
            .filter(|&x| x > 0)
            .map(|x| if x % 2 == 0 { 2 } else { 1 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimum_length("abaacbcbb".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_length("aa".to_string()));
    }
}
