pub struct Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut f = [0; 101];
        for x in nums {
            f[x as usize] += 1;
            if f[x as usize] > 2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_possible_to_split(vec![1, 1, 1, 1]));
    }
}
