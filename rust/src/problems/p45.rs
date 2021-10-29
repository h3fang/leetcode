pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut furthest = nums[0] as usize;
        let mut p = 0;
        let mut r = 0;
        while p < n - 1 {
            let mut next_furthest = furthest;
            for (i, e) in nums[..=furthest.min(n - 1)].iter().enumerate().skip(p + 1) {
                next_furthest = next_furthest.max(i + *e as usize);
            }
            p = furthest;
            furthest = next_furthest;
            r += 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }
}
