pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        fn ts(k: usize, conditions: &[Vec<i32>]) -> Vec<usize> {
            let mut g = vec![vec![]; k + 1];
            let mut indegree = vec![0; k + 1];
            for c in conditions {
                g[c[0] as usize].push(c[1]);
                indegree[c[1] as usize] += 1;
            }
            let mut q = VecDeque::new();
            let mut result = vec![0; k];
            for (i, &d) in indegree.iter().enumerate().skip(1) {
                if d == 0 {
                    q.push_back(i);
                }
            }
            let mut curr = 0;
            while let Some(i) = q.pop_front() {
                result[i - 1] = curr;
                curr += 1;
                for &child in &g[i] {
                    let c = child as usize;
                    indegree[c] -= 1;
                    if indegree[c] == 0 {
                        q.push_back(c);
                    }
                }
            }
            if curr != k { vec![] } else { result }
        }

        let k = k as usize;
        let row = ts(k, &row_conditions);
        if row.is_empty() {
            return vec![];
        }
        let col = ts(k, &col_conditions);
        if col.is_empty() {
            return vec![];
        }

        let mut result = vec![vec![0; k]; k];
        for c in 1..=k {
            result[row[c - 1]][col[c - 1]] = c as i32;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn parse_condition(c: &[[i32; 2]]) -> Vec<Vec<i32>> {
        c.iter().map(|c| c.to_vec()).collect::<Vec<_>>()
    }

    fn check(k: i32, row_conditions: &[[i32; 2]], col_conditions: &[[i32; 2]]) {
        let rc = parse_condition(row_conditions);
        let cc = parse_condition(col_conditions);
        let result = Solution::build_matrix(k, rc, cc);

        let mut m = HashMap::new();
        for (i, r) in result.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 0 {
                    continue;
                }
                assert!(!m.contains_key(&c));
                m.insert(c, (i, j));
            }
        }

        for c in row_conditions {
            let r1 = m[&c[0]].0;
            let r2 = m[&c[1]].0;
            assert!(r1 < r2);
        }

        for c in col_conditions {
            let c1 = m[&c[0]].1;
            let c2 = m[&c[1]].1;
            assert!(c1 < c2);
        }
    }

    #[test]
    fn case1() {
        let k = 3;
        let row_conditions = [[1, 2], [3, 2]];
        let col_conditions = [[2, 1], [3, 2]];
        check(k, &row_conditions, &col_conditions);
    }

    #[test]
    fn case2() {
        let k = 3;
        let row_conditions = [[1, 2], [2, 3], [3, 1], [2, 3]];
        let col_conditions = [[2, 1]];
        let rc = parse_condition(&row_conditions);
        let cc = parse_condition(&col_conditions);
        let result = Solution::build_matrix(k, rc, cc);
        assert!(result.is_empty());
    }
}
