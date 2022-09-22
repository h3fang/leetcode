pub struct Solution;

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even = 0;
        for n in &nums {
            if n % 2 == 0 {
                even += n;
            }
        }
        queries
            .into_iter()
            .map(|q| {
                let v = q[0];
                let i = q[1] as usize;
                if nums[i] % 2 == 0 {
                    even -= nums[i];
                }
                nums[i] += v;
                if nums[i] % 2 == 0 {
                    even += nums[i];
                }
                even
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3, 4];
        let queries = [[1, 0], [-3, 1], [-4, 0], [2, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![8, 6, 2, 4],
            Solution::sum_even_after_queries(nums, queries)
        );
    }

    #[test]
    fn case2() {
        let nums = vec![1];
        let queries = [[4, 0]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![0], Solution::sum_even_after_queries(nums, queries));
    }
}
