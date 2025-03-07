pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            }
            _ => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
        }
    }
}

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dist = vec![vec![-1; n]; n];
        let mut q = vec![];
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    q.push((i as i32, j as i32));
                    dist[i][j] = 0;
                }
            }
        }
        let mut groups = vec![];
        while !q.is_empty() {
            let mut next = vec![];
            for (i, j) in q {
                for (i, j) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if i < 0
                        || j < 0
                        || i >= n as i32
                        || j >= n as i32
                        || dist[i as usize][j as usize] >= 0
                    {
                        continue;
                    }
                    next.push((i, j));
                    dist[i as usize][j as usize] = groups.len() as i32 + 1;
                }
            }
            groups.push(next.clone());
            q = next;
        }
        let mut dsu = Dsu::new(n * n);
        for (d, g) in groups.into_iter().enumerate().rev().skip(1) {
            for (i, j) in g {
                for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if i1 < 0
                        || j1 < 0
                        || i1 >= n as i32
                        || j1 >= n as i32
                        || dist[i1 as usize][j1 as usize] < dist[i as usize][j as usize]
                    {
                        continue;
                    }
                    dsu.union(i as usize * n + j as usize, i1 as usize * n + j1 as usize);
                }
            }
            if dsu.find(0) == dsu.find(n * n - 1) {
                return d as i32 + 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0, 0], [0, 0, 0], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(0, Solution::maximum_safeness_factor(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0, 1], [0, 0, 0], [0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::maximum_safeness_factor(grid));
    }

    #[test]
    fn case3() {
        let grid = [[0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [1, 0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::maximum_safeness_factor(grid));
    }
}
