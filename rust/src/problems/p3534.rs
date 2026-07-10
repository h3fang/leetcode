pub struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_unstable_by_key(|&i| nums[i]);

        let mut rank = vec![0; n];
        let max = (usize::BITS - n.leading_zeros()) as usize;
        let mut p = vec![vec![0; max]; n];
        let mut l = 0;
        for (i, &j) in idx.iter().enumerate() {
            rank[j] = i;
            while nums[j] - nums[idx[l]] > max_diff {
                l += 1;
            }
            p[i][0] = l;
        }

        for i in 0..max - 1 {
            for x in 0..n {
                let pa = p[x][i];
                p[x][i + 1] = p[pa][i];
            }
        }

        queries
            .into_iter()
            .map(|q| {
                if q[0] == q[1] {
                    0
                } else {
                    let (mut l, mut r) = (rank[q[0] as usize], rank[q[1] as usize]);
                    if l > r {
                        (l, r) = (r, l);
                    }

                    let mut ans = 0;
                    for k in (0..max).rev() {
                        if p[r][k] > l {
                            ans |= 1 << k;
                            r = p[r][k];
                        }
                    }

                    if p[r][0] > l { -1 } else { ans + 1 }
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 8, 3, 4, 2];
        let queries = [[0, 3], [2, 4]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![1, 1],
            Solution::path_existence_queries(5, nums, 3, queries)
        );
    }

    #[test]
    fn case2() {
        let nums = vec![5, 3, 1, 9, 10];
        let queries = [[0, 1], [0, 2], [2, 3], [4, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![1, 2, -1, 1],
            Solution::path_existence_queries(5, nums, 2, queries)
        );
    }

    #[test]
    fn case3() {
        let nums = vec![3, 6, 1];
        let queries = [[0, 0], [0, 1], [1, 2]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![0, -1, -1],
            Solution::path_existence_queries(3, nums, 1, queries)
        );
    }
}
