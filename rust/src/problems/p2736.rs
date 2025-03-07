pub struct Solution;

impl Solution {
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut nums = nums1.into_iter().zip(nums2).collect::<Vec<_>>();
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_unstable_by_key(|e| -e.0);
        queries.sort_unstable_by_key(|e| -e.1[0]);
        let n = nums.len();
        let mut s: Vec<(i32, i32)> = Vec::with_capacity(n);
        let mut result = vec![-1; queries.len()];
        let mut j = 0;
        for (i, q) in queries.into_iter() {
            while j < n && nums[j].0 >= q[0] {
                let (a, b) = nums[j];
                if s.is_empty() || s.last().unwrap().0 < b {
                    while !s.is_empty() && s.last().unwrap().1 < a + b {
                        s.pop();
                    }
                    s.push((b, a + b));
                }
                j += 1;
            }
            let k = s.partition_point(|e| e.0 < q[1]);
            if k < s.len() {
                result[i] = s[k].1;
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
        let nums1 = vec![4, 3, 1, 2];
        let nums2 = vec![2, 4, 9, 5];
        let queries = [[4, 1], [1, 3], [2, 5]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![6, 10, 7],
            Solution::maximum_sum_queries(nums1, nums2, queries)
        );
    }

    #[test]
    fn case2() {
        let nums1 = vec![3, 2, 5];
        let nums2 = vec![2, 3, 4];
        let queries = [[4, 4], [3, 2], [1, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![9, 9, 9],
            Solution::maximum_sum_queries(nums1, nums2, queries)
        );
    }

    #[test]
    fn case3() {
        let nums1 = vec![2, 1];
        let nums2 = vec![2, 3];
        let queries = [[3, 3]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![-1],
            Solution::maximum_sum_queries(nums1, nums2, queries)
        );
    }
}
