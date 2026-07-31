pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let w = word.as_bytes();
        let mut freq = [0; 26];
        for b in w {
            freq[(b - b'a') as usize] += 1;
        }
        freq.sort_unstable_by_key(|e| -e);
        let p = freq.partition_point(|&e| e > 0);
        freq[..p]
            .chunks(8)
            .enumerate()
            .map(|(i, c)| c.iter().sum::<i32>() * (i as i32 + 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::minimum_pushes("xyzxyzxyzxyz".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            24,
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string())
        );
    }
}
