pub struct Solution;

struct DisjointSetUnion {
    parent: Vec<usize>,
    rank: Vec<usize>,
    merged: usize,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            merged: 0,
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

        self.merged += 1;

        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Equal => {
                self.parent[px] = py;
                self.rank[px] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[py] = px,
        }
    }
}

impl Solution {
    pub fn num_islands_union_find(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dsu = DisjointSetUnion::new(m * n + 1);

        let mut ones = 0;

        let index = |i: usize, j: usize| i * n + j;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    if i + 1 < m && grid[i + 1][j] == '1' {
                        dsu.union(index(i, j), index(i + 1, j));
                    }

                    if j + 1 < n && grid[i][j + 1] == '1' {
                        dsu.union(index(i, j), index(i, j + 1));
                    }
                    ones += 1;
                }
            }
        }

        ones - dsu.merged as i32
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if grid[i][j] != '1' {
                return;
            }
            grid[i][j] = '0';

            if i + 1 < grid.len() {
                dfs(grid, i + 1, j);
            }

            if i > 0 {
                dfs(grid, i - 1, j);
            }

            if j + 1 < grid[0].len() {
                dfs(grid, i, j + 1);
            }

            if j > 0 {
                dfs(grid, i, j - 1);
            }
        }
        let m = grid.len();
        let n = grid[0].len();

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    result += 1;
                    dfs(&mut grid, i, j);
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
        let grid = [
            ["1", "1", "1", "1", "0"],
            ["1", "1", "0", "1", "0"],
            ["1", "1", "0", "0", "0"],
            ["0", "0", "0", "0", "0"],
        ];
        let grid = grid
            .into_iter()
            .map(|r| {
                r.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(1, Solution::num_islands_union_find(grid.clone()));
        assert_eq!(1, Solution::num_islands(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            ["1", "1", "0", "0", "0"],
            ["1", "1", "0", "0", "0"],
            ["0", "0", "1", "0", "0"],
            ["0", "0", "0", "1", "1"],
        ];
        let grid = grid
            .into_iter()
            .map(|r| {
                r.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(3, Solution::num_islands_union_find(grid.clone()));
        assert_eq!(3, Solution::num_islands(grid));
    }

    #[test]
    fn case3() {
        let grid = [["1", "1", "1"], ["0", "1", "0"], ["1", "1", "1"]];
        let grid = grid
            .into_iter()
            .map(|r| {
                r.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(1, Solution::num_islands_union_find(grid.clone()));
        assert_eq!(1, Solution::num_islands(grid));
    }
}
