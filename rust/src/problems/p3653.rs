pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        for q in queries {
            for i in (q[0] as usize..=q[1] as usize).step_by(q[2] as usize) {
                nums[i] = ((nums[i] as i64 * q[3] as i64) % MOD) as i32;
            }
        }
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 1, 1];
        let queries = [[0, 2, 1, 4]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(4, Solution::xor_after_queries(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![2, 3, 1, 5, 4];
        let queries = [[1, 4, 2, 3], [0, 2, 1, 2]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(31, Solution::xor_after_queries(nums, queries));
    }
}
