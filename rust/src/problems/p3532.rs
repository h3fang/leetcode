pub struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut id = vec![0; n as usize];

        for (i, w) in nums.windows(2).enumerate() {
            id[i + 1] = id[i];
            if w[1] - w[0] > max_diff {
                id[i + 1] += 1;
            }
        }

        queries
            .into_iter()
            .map(|q| id[q[0] as usize] == id[q[1] as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 3];
        let queries = [[0, 0], [0, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![true, false],
            Solution::path_existence_queries(2, nums, 1, queries)
        );
    }

    #[test]
    fn case2() {
        let nums = vec![2, 5, 6, 8];
        let queries = [[0, 1], [0, 2], [1, 3], [2, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![false, false, true, true],
            Solution::path_existence_queries(4, nums, 2, queries)
        );
    }
}
