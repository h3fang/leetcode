pub struct Solution;

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let mut sum = vec![0; n + 1];
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut prev = -1;
        for (i, b) in s.as_bytes().iter().enumerate() {
            if *b == b'|' {
                prev = i as i64;
                sum[i + 1] = sum[i];
            } else {
                sum[i + 1] = sum[i] + 1;
            }
            left[i] = prev;
        }
        prev = n as i64;
        for (i, b) in s.as_bytes().iter().enumerate().rev() {
            if *b == b'|' {
                prev = i as i64;
            }
            right[i] = prev;
        }
        queries
            .iter()
            .map(|q| {
                let a = right[q[0] as usize];
                let b = left[q[1] as usize];
                if a >= b {
                    0
                } else {
                    sum[b as usize] - sum[a as usize]
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
        let s = "**|**|***|".to_string();
        let queries = [[2, 5], [5, 9]];
        let queries = queries.iter().map(|q| q.to_vec()).collect::<Vec<_>>();
        assert_eq!(vec![2, 3], Solution::plates_between_candles(s, queries));
    }

    #[test]
    fn case2() {
        let s = "***|**|*****|**||**|*".to_string();
        let queries = [[1, 17], [4, 5], [14, 17], [5, 11], [15, 16]];
        let queries = queries.iter().map(|q| q.to_vec()).collect::<Vec<_>>();
        assert_eq!(
            vec![9, 0, 0, 0, 0],
            Solution::plates_between_candles(s, queries)
        );
    }
}
