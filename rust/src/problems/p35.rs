pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut a = 0i32;
        let mut b = nums.len() as i32 - 1;

        let mut result = b + 1;

        while a <= b {
            let m = a + (b - a) / 2;
            let n = nums[m as usize];
            if n >= target {
                result = m;
                b = m - 1;
            } else {
                a = m + 1;
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
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }

    #[test]
    fn case5() {
        assert_eq!(0, Solution::search_insert(vec![1], 0));
    }
}
