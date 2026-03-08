pub struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut s = Vec::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            s.push(((num.as_bytes()[i] - b'0') ^ 1) + b'0');
        }
        unsafe { String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(nums: &[&str]) {
        let nums = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        let result = Solution::find_different_binary_string(nums.clone());
        assert!(result.len() == nums.len());
        assert!(u16::from_str_radix(&result, 2).is_ok());
        assert!(!nums.contains(&result));
    }

    #[test]
    fn case1() {
        check(&["01", "10"]);
    }

    #[test]
    fn case2() {
        check(&["00", "01"]);
    }

    #[test]
    fn case3() {
        check(&["111", "011", "001"]);
    }

    #[test]
    fn case4() {
        check(&["001011", "111110", "010110", "010010", "101111", "011001"]);
    }
}
