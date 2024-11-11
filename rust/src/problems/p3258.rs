pub struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let (mut l, mut count, mut result) = (0, [0; 2], 0);
        for (r, &b) in s.iter().enumerate() {
            count[(b - b'0') as usize] += 1;
            while count[0] > k && count[1] > k {
                count[(s[l] - b'0') as usize] -= 1;
                l += 1;
            }
            result += r - l + 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            12,
            Solution::count_k_constraint_substrings("10101".to_string(), 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            25,
            Solution::count_k_constraint_substrings("1010101".to_string(), 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            15,
            Solution::count_k_constraint_substrings("11111".to_string(), 1)
        );
    }
}
