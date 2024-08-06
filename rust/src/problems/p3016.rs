pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let w = word.as_bytes();
        let mut freq = [0; 26];
        for b in w {
            freq[(b - b'a') as usize] += 1;
        }
        freq.sort_unstable_by_key(|e| -e);
        let mut result = 0;
        for (i, c) in freq.chunks(8).enumerate() {
            for &f in c {
                if f == 0 {
                    return result;
                }
                result += (i as i32 + 1) * f;
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
