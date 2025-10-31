pub struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let (mut a, mut b) = (0, 0);
        for x in nums {
            a += x;
            b += x * x;
        }
        a -= (n - 1) * n / 2;
        b -= (n * 2 - 1) * (n - 1) * n / 6;
        let x = (a - (2 * b - a * a).isqrt()) / 2;
        vec![x, a - x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ans = Solution::get_sneaky_numbers(vec![0, 1, 1, 0]);
        ans.sort_unstable();
        assert_eq!(vec![0, 1], ans);
    }

    #[test]
    fn case2() {
        let mut ans = Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]);
        ans.sort_unstable();
        assert_eq!(vec![2, 3], ans);
    }

    #[test]
    fn case3() {
        let mut ans = Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]);
        ans.sort_unstable();
        assert_eq!(vec![4, 5], ans);
    }
}
