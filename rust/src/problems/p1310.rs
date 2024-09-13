pub struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pre_sum = vec![0; arr.len() + 1];
        for (i, &x) in arr.iter().enumerate() {
            pre_sum[i + 1] = pre_sum[i] ^ x;
        }
        queries
            .into_iter()
            .map(|q| pre_sum[q[1] as usize + 1] ^ pre_sum[q[0] as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = vec![1, 3, 4, 8];
        let queries = [[0, 1], [1, 2], [0, 3], [3, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![2, 7, 14, 8], Solution::xor_queries(arr, queries));
    }

    #[test]
    fn case2() {
        let arr = vec![4, 8, 2, 10];
        let queries = [[2, 3], [1, 3], [0, 0], [0, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![8, 0, 4, 4], Solution::xor_queries(arr, queries));
    }
}
