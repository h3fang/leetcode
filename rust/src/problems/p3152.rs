pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut f = vec![1; n];
        for (i, w) in nums.windows(2).enumerate() {
            if w[1] % 2 != w[0] % 2 {
                f[i + 1] = f[i] + 1;
            }
        }
        queries
            .into_iter()
            .map(|q| {
                let (i, j) = (q[0] as usize, q[1] as usize);
                f[j] > j - i
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[0, 4]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![false],
            Solution::is_array_special(vec![3, 4, 1, 2, 6], queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[0, 2], [2, 3]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![false, true],
            Solution::is_array_special(vec![4, 3, 1, 6], queries)
        );
    }
}
