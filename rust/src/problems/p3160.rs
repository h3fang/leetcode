pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = queries.len();
        let mut balls = HashMap::with_capacity(n);
        let mut colors = HashMap::with_capacity(n);
        let mut result = Vec::with_capacity(queries.len());
        for q in queries {
            if let Some(c) = balls.insert(q[0], q[1]) {
                let e = colors.entry(c).or_default();
                *e -= 1;
                if *e == 0 {
                    colors.remove(&c);
                }
            }
            *colors.entry(q[1]).or_insert(0) += 1;
            result.push(colors.len() as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[1, 4], [2, 5], [1, 3], [3, 4]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![1, 2, 2, 3], Solution::query_results(4, queries));
    }

    #[test]
    fn case2() {
        let queries = [[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![1, 2, 2, 3, 4], Solution::query_results(4, queries));
    }
}
