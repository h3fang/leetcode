pub struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut min = [i32::MAX; 2];
        for x in nums1 {
            let p = (x % 2) as usize;
            min[p] = min[p].min(x);
        }

        min[1] == i32::MAX || min[0] > min[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::uniform_array(vec![1, 4, 7]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::uniform_array(vec![2, 3]));
    }

    #[test]
    fn case3() {
        assert!(Solution::uniform_array(vec![4, 6]));
    }
}
