pub struct Solution;

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut pre = vec![0i32; s.len() + 1];
        for (i, &b) in s.as_bytes().iter().enumerate() {
            pre[i + 1] = pre[i] ^ (1 << (b - b'a'));
        }
        queries
            .iter()
            .map(|q| {
                let x = pre[q[1] as usize + 1] ^ pre[q[0] as usize];
                x.count_ones() as i32 <= q[2] * 2 + 1
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![true, false, false, true, true],
            Solution::can_make_pali_queries("abcda".to_string(), queries)
        );
    }
}
