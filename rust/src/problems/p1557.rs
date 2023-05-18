pub struct Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indeg = vec![0; n as usize];
        for e in edges {
            indeg[e[1] as usize] += 1;
        }
        indeg
            .into_iter()
            .enumerate()
            .filter_map(|(i, d)| if d == 0 { Some(i as i32) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let mut result = Solution::find_smallest_set_of_vertices(6, edges);
        result.sort_unstable();
        assert_eq!(vec![0, 3], result);
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let mut result = Solution::find_smallest_set_of_vertices(5, edges);
        result.sort_unstable();
        assert_eq!(vec![0, 2, 3], result);
    }
}
