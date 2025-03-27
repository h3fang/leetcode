pub struct Solution;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let (mut dominant, mut vote) = (nums[0], 1);
        for &x in nums.iter().skip(1) {
            if x == dominant {
                vote += 1;
            } else {
                vote -= 1;
                if vote <= 0 {
                    dominant = x;
                    vote = 1;
                }
            }
        }
        let total = nums.iter().filter(|&&x| x == dominant).count();
        let (n, mut c) = (nums.len(), 0);
        for (i, &x) in nums.iter().enumerate().take(n - 1) {
            if x == dominant {
                c += 1;
            }
            if c * 2 > i + 1 && (total - c) * 2 > (n - i - 1) {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_index(vec![1, 2, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::minimum_index(vec![3, 3, 3, 3, 7, 2, 2]));
    }
}
