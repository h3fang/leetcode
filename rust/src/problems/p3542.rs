pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut s = Vec::with_capacity(nums.len() + 1);
        s.push(0);

        let mut ans = 0;
        for x in nums {
            while *s.last().unwrap() > x {
                s.pop();
            }
            if *s.last().unwrap() != x {
                ans += 1;
                s.push(x);
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
        assert_eq!(1, Solution::min_operations(vec![0, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_operations(vec![3, 1, 2, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::min_operations(vec![1, 2, 1, 2, 1, 2]));
    }
}
