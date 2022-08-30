pub struct Solution;

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut q = queries
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .collect::<Vec<_>>();
        q.sort_unstable_by_key(|e| e.0);
        let mut result = vec![0; queries.len()];
        let mut sum = 0;
        let mut i = 0;
        for (c, j) in q {
            while i < nums.len() && nums[i] + sum <= c {
                sum += nums[i];
                i += 1;
            }
            result[j] = i as i32;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![4, 5, 2, 1];
        let queries = vec![3, 10, 21];
        assert_eq!(vec![2, 3, 4], Solution::answer_queries(nums, queries));
    }
}
