pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;
        let mut g = vec![vec![]; n];
        let mut in_deg = vec![0; n];
        for p in &prerequisites {
            g[p[0] as usize].push(p[1]);
            in_deg[p[1] as usize] += 1;
        }
        let mut q = VecDeque::new();
        in_deg
            .iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .for_each(|(i, _)| q.push_back(i));
        let mut is_req = vec![vec![false; n]; n];
        while let Some(i) = q.pop_back() {
            for &j in &g[i] {
                let j = j as usize;
                is_req[i][j] = true;
                for r in &mut is_req {
                    r[j] = r[i] || r[j];
                }
                in_deg[j] -= 1;
                if in_deg[j] == 0 {
                    q.push_back(j);
                }
            }
        }
        queries
            .into_iter()
            .map(|q| is_req[q[0] as usize][q[1] as usize])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let prerequisites = [[1, 0]].iter().map(|p| p.to_vec()).collect();
        let queries = [[0, 1], [1, 0]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(
            vec![false, true],
            Solution::check_if_prerequisite(2, prerequisites, queries)
        );
    }

    #[test]
    fn case2() {
        let prerequisites = vec![];
        let queries = [[1, 0], [0, 1]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(
            vec![false, false],
            Solution::check_if_prerequisite(2, prerequisites, queries)
        );
    }

    #[test]
    fn case3() {
        let prerequisites = [[1, 2], [1, 0], [2, 0]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        let queries = [[1, 0], [1, 2]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(
            vec![true, true],
            Solution::check_if_prerequisite(3, prerequisites, queries)
        );
    }
}
