pub struct Solution;

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        while n > 1 {
            for i in 0..n / 2 {
                let a = nums[2 * i];
                let b = nums[2 * i + 1];
                if i % 2 == 0 {
                    nums[i] = a.min(b);
                } else {
                    nums[i] = a.max(b);
                }
            }
            n /= 2;
        }
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_max_game(vec![3]));
    }
}
