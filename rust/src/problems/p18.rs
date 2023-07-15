pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 4 {
            return vec![];
        }
        nums.sort_unstable();
        let mut result = vec![];
        let target = target as i64;
        for i in 0..n - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let a = nums[i] as i64;
            if target < a + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64 {
                break;
            }
            for j in i + 1..n - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let b = nums[j] as i64;
                if target < a + b + nums[j + 1] as i64 + nums[j + 2] as i64 {
                    break;
                }
                let t = target - a - b;
                let (mut left, mut right) = (j + 1, n - 1);
                while left < right {
                    match (nums[left] as i64 + nums[right] as i64).cmp(&t) {
                        std::cmp::Ordering::Less => left += 1,
                        std::cmp::Ordering::Greater => right -= 1,
                        std::cmp::Ordering::Equal => {
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            while left < right && nums[left] == nums[left + 1] {
                                left += 1;
                            }
                            left += 1;
                            while left < right && nums[right] == nums[right - 1] {
                                right -= 1;
                            }
                            right -= 1;
                        }
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
        let mut result = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        result.sort_unstable();
        let mut expected = [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::four_sum(vec![2, 2, 2, 2, 2], 8);
        result.sort_unstable();
        let mut expected = [[2, 2, 2, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
