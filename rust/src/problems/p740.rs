pub struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let mut sum = vec![0; max as usize + 1];
        for n in nums {
            sum[n as usize] += n;
        }

        let mut pprev = 0;
        let mut prev = 0;
        for n in sum {
            let temp = prev;
            prev = (pprev + n).max(prev);
            pprev = temp;
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::delete_and_earn(vec![3, 4, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]));
    }
}
