pub struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut i = -1;
        let mut j = -1;

        for (k, &n) in nums.iter().enumerate() {
            if n < min {
                min = n;
                i = k as i32;
            }

            if n > max {
                max = n;
                j = k as i32;
            }
        }

        let n = nums.len() as i32;
        let (i, j) = if i < j { (i, j) } else { (j, i) };

        let a = n - i;
        let b = j + 1;
        let c = n - (j - i - 1);
        a.min(b).min(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            5,
            Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::minimum_deletions(vec![-11000]));
    }
}
