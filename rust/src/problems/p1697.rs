pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[py] = px,
            std::cmp::Ordering::Equal => {
                self.parent[py] = px;
                self.rank[px] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[px] = py,
        }
    }
}

impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        mut edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut dsu = Dsu::new(n as usize);
        edge_list.sort_unstable_by_key(|e| e[2]);
        let mut q = (0..queries.len()).collect::<Vec<_>>();
        q.sort_unstable_by_key(|&i| queries[i][2]);
        let mut k = 0;
        let mut result = vec![false; queries.len()];
        q.into_iter().for_each(|i| {
            while k < edge_list.len() && edge_list[k][2] < queries[i][2] {
                dsu.union(edge_list[k][0] as usize, edge_list[k][1] as usize);
                k += 1;
            }
            result[i] = dsu.find(queries[i][0] as usize) == dsu.find(queries[i][1] as usize);
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let edge_list = [[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        let queries = [[0, 1, 2], [0, 2, 5]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![false, true],
            Solution::distance_limited_paths_exist(n, edge_list, queries)
        );
    }

    #[test]
    fn case2() {
        let n = 5;
        let edge_list = [[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        let queries = [[0, 4, 14], [1, 4, 13]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![true, false],
            Solution::distance_limited_paths_exist(n, edge_list, queries)
        );
    }
}
