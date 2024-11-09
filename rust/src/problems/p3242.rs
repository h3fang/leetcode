pub struct NeighborSum {
    adj: Vec<i32>,
    diag: Vec<i32>,
}

impl NeighborSum {
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let mut adj = vec![0; n * n];
        let mut diag = vec![0; n * n];
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                adj[c as usize] = if i > 0 { grid[i - 1][j] } else { 0 }
                    + if j > 0 { grid[i][j - 1] } else { 0 }
                    + if i + 1 < n { grid[i + 1][j] } else { 0 }
                    + if j + 1 < n { grid[i][j + 1] } else { 0 };
                diag[c as usize] = if i > 0 && j > 0 {
                    grid[i - 1][j - 1]
                } else {
                    0
                } + if i > 0 && j + 1 < n {
                    grid[i - 1][j + 1]
                } else {
                    0
                } + if i + 1 < n && j > 0 {
                    grid[i + 1][j - 1]
                } else {
                    0
                } + if i + 1 < n && j + 1 < n {
                    grid[i + 1][j + 1]
                } else {
                    0
                };
            }
        }
        Self { adj, diag }
    }

    pub fn adjacent_sum(&self, value: i32) -> i32 {
        self.adj[value as usize]
    }

    pub fn diagonal_sum(&self, value: i32) -> i32 {
        self.diag[value as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let ns = NeighborSum::new(grid);
        assert_eq!(6, ns.adjacent_sum(1));
        assert_eq!(16, ns.adjacent_sum(4));
        assert_eq!(16, ns.diagonal_sum(4));
        assert_eq!(4, ns.diagonal_sum(8));
    }

    #[test]
    fn case2() {
        let grid = [[1, 2, 0, 3], [4, 7, 15, 6], [8, 9, 10, 11], [12, 13, 14, 5]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let ns = NeighborSum::new(grid);
        assert_eq!(23, ns.adjacent_sum(15));
        assert_eq!(45, ns.diagonal_sum(9));
    }
}
