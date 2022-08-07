pub struct Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let n = nums.len();
        let mut result = 0;
        for i in 0..n {
            for j in i + 1..n {
                if nums[j] - nums[i] != diff {
                    continue;
                }
                for k in j + 1..n {
                    if nums[k] - nums[j] != diff {
                        continue;
                    }
                    result += 1;
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
        assert_eq!(2, Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3))
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2))
    }
}
