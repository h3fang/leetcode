pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut next: Vec<i32> = (1..n).collect();
        let mut len = n - 1;
        queries
            .into_iter()
            .map(|q| {
                let (mut l, r) = (q[0] as usize, q[1]);
                while next[l] < r {
                    len -= 1;
                    (next[l], l) = (r, next[l] as usize);
                }
                len
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[2, 4], [0, 2], [0, 4]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![3, 2, 1],
            Solution::shortest_distance_after_queries(5, queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[0, 3], [0, 2]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![1, 1],
            Solution::shortest_distance_after_queries(4, queries)
        );
    }
}
