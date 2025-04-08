pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut seen = [false; 101];
        let mut ans = 0;
        for x in nums {
            if x < k {
                return -1;
            } else if x > k && !seen[x as usize] {
                ans += 1;
                seen[x as usize] = true;
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
        assert_eq!(2, Solution::min_operations(vec![5, 2, 5, 4, 5], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_operations(vec![2, 1, 2], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::min_operations(vec![9, 7, 5, 3], 1));
    }
}
