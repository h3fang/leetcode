pub struct Solution;

use std::collections::HashSet;

#[derive(Clone)]
struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<i32>,
    max: i32,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            max: 0,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let px = self.parent[x];
        if px != x {
            self.parent[x] = self.find(px);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return;
        }

        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
                self.size[py] += self.size[px];
                self.max = self.max.max(self.size[py]);
            }
            _ => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
                self.max = self.max.max(self.size[px]);
            }
        }
    }
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dsu = DisjointSetUnion::new(n * n);
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 1 {
                    if i + 1 < n && grid[i + 1][j] == 1 {
                        dsu.union(i * n + j, (i + 1) * n + j);
                    }
                    if j + 1 < n && grid[i][j + 1] == 1 {
                        dsu.union(i * n + j, i * n + j + 1);
                    }
                }
            }
        }

        let mut result = dsu.max.max(1);
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 0 {
                    let mut size = 1;
                    let mut roots = HashSet::new();
                    for (di, dj) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                        let i1 = i as i32 + di;
                        let j1 = j as i32 + dj;
                        if i1 < 0
                            || j1 < 0
                            || i1 >= n as i32
                            || j1 >= n as i32
                            || grid[i1 as usize][j1 as usize] == 0
                        {
                            continue;
                        }
                        let x = i1 as usize * n + j1 as usize;
                        let px = dsu.find(x);
                        if roots.contains(&px) {
                            continue;
                        }
                        roots.insert(px);
                        size += dsu.size[px];
                    }
                    result = result.max(size);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0], [0, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::largest_island(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 1], [1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::largest_island(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 1], [1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::largest_island(grid));
    }
}
