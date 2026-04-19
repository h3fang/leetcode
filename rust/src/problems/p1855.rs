pub struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut j = nums2.len() as i32 - 1;
        for (i, x) in nums1.into_iter().enumerate().rev() {
            while j >= 0 && nums2[j as usize] < x {
                j -= 1;
            }
            ans = ans.max(j - i as i32);
            if j < 0 {
                break;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_distance(vec![2, 2, 2], vec![10, 10, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::max_distance(vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25])
        );
    }
}
