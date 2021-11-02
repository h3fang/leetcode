pub struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![[1; 2]; nums.len() + 1];
        let mut max = 0;
        let mut r = 0;
        for (i, &n) in nums.iter().enumerate() {
            for j in 0..i {
                if n > nums[j] {
                    match (1 + dp[j][0]).cmp(&dp[i][0]) {
                        std::cmp::Ordering::Less => {}
                        std::cmp::Ordering::Equal => dp[i][1] += dp[j][1],
                        std::cmp::Ordering::Greater => {
                            dp[i][0] = 1 + dp[j][0];
                            dp[i][1] = dp[j][1];
                        }
                    }
                }
            }
            match dp[i][0].cmp(&max) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => r += dp[i][1],
                std::cmp::Ordering::Greater => {
                    max = dp[i][0];
                    r = dp[i][1];
                }
            }
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::find_number_of_lis(vec![0]));
    }
}
