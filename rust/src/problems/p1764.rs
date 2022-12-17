pub struct Solution;

impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;
        for g in groups {
            let mut matched = false;
            let n = g.len();
            while i + n <= nums.len() {
                if g == nums[i..i + n] {
                    matched = true;
                    i += n;
                    break;
                }
                i += 1;
            }
            if !matched {
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
        let groups = vec![vec![1, -1, -1], vec![3, -2, 0]];
        let nums = vec![1, -1, 0, 1, -1, -1, 3, -2, 0];
        assert!(Solution::can_choose(groups, nums));
    }

    #[test]
    fn case2() {
        let groups = vec![vec![10, -2], vec![1, 2, 3, 4]];
        let nums = vec![1, 2, 3, 4, 10, -2];
        assert!(!Solution::can_choose(groups, nums));
    }

    #[test]
    fn case3() {
        let groups = vec![vec![1, 2, 3], vec![3, 4]];
        let nums = vec![7, 7, 1, 2, 3, 4, 7, 7];
        assert!(!Solution::can_choose(groups, nums));
    }
}
