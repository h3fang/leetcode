pub struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0]
            + nums
                .windows(2)
                .take_while(|w| w[0] + 1 == w[1])
                .map(|w| w[1])
                .sum::<i32>();

        let mut f = 0u64;
        for x in nums {
            f |= 1 << x;
        }

        while sum < 51 && f & (1 << sum) > 0 {
            sum += 1;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::missing_integer(vec![1, 2, 3, 2, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]));
    }
}
