pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = i32::MAX;

        let mut p = 0;

        for (i, &x) in nums.iter().enumerate().skip(1) {
            if x < nums[i - 1] {
                if p != 0 {
                    p = -1;
                    break;
                }
                p = i as i32;
            }
        }

        if p == 0 {
            return 0;
        } else if p != -1 && nums[0] > nums[n - 1] {
            ans = p.min(n as i32 - p + 2);
        }

        p = 0;

        for (i, &x) in nums.iter().enumerate().skip(1) {
            if x > nums[i - 1] {
                if p != 0 {
                    p = -1;
                    break;
                }
                p = i as i32;
            }
        }

        if p == 0 {
            return 1;
        } else if p != -1 && nums[0] < nums[n - 1] {
            ans = ans.min(p + 1).min(n as i32 - p + 1);
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_operations(vec![0, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_operations(vec![1, 0, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_operations(vec![2, 0, 1, 3]));
    }
}
