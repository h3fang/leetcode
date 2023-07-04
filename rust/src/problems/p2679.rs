pub struct Solution;

impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.iter_mut().for_each(|r| r.sort_unstable());
        (0..nums[0].len())
            .map(|j| nums.iter().map(|r| r[j]).max().unwrap())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [[7, 2, 1], [6, 4, 2], [6, 5, 3], [3, 2, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(15, Solution::matrix_sum(nums));
    }

    #[test]
    fn case2() {
        let nums = [[1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::matrix_sum(nums));
    }
}
