pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut result = i32::MAX / 2;
        for (i, &a) in nums.iter().enumerate() {
            if i > 0 && a == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                match (a + nums[j] + nums[k]).cmp(&target) {
                    std::cmp::Ordering::Less => {
                        if (target - result).abs() > (target - (a + nums[j] + nums[k])).abs() {
                            result = a + nums[j] + nums[k];
                        }
                        j += 1;
                    }
                    std::cmp::Ordering::Equal => return target,
                    std::cmp::Ordering::Greater => {
                        if (target - result).abs() > (target - (a + nums[j] + nums[k])).abs() {
                            result = a + nums[j] + nums[k];
                        }
                        k -= 1;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::three_sum_closest(vec![0, 0, 0], 1));
    }
}
