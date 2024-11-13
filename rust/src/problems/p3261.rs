pub struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = s.len();
        let s = s.as_bytes();
        let (mut left, mut presum) = (vec![0; n], vec![0; n + 1]);
        let (mut l, mut count) = (0, [0; 2]);
        for (r, &b) in s.iter().enumerate() {
            count[(b - b'0') as usize] += 1;
            while count[0] > k && count[1] > k {
                count[(s[l] - b'0') as usize] -= 1;
                l += 1;
            }
            left[r] = l;
            presum[r + 1] = presum[r] + r - l + 1;
        }
        queries
            .into_iter()
            .map(|q| {
                let (l, r) = (q[0] as usize, q[1] as usize);
                let j = left[l..=r].partition_point(|&x| x < l) + l;
                (presum[r + 1] - presum[j] + (j - l + 1) * (j - l) / 2) as i64
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[0, 6]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![26],
            Solution::count_k_constraint_substrings("0001111".to_string(), 2, queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[0, 5], [1, 4], [2, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![15, 9, 3],
            Solution::count_k_constraint_substrings("010101".to_string(), 1, queries)
        );
    }
}
