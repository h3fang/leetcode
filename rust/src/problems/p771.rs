pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut m = [false; 256];
        for &c in jewels.as_bytes() {
            m[c as usize] = true;
        }
        stones
            .as_bytes()
            .iter()
            .map(|&b| i32::from(m[b as usize]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string())
        );
    }
}
