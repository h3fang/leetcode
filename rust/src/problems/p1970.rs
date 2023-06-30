pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<u32>,
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
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
            }
            std::cmp::Ordering::Equal => {
                self.rank[py] += 1;
                self.parent[px] = py;
            }
            std::cmp::Ordering::Greater => self.parent[py] = px,
        }
    }
}

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let n = (row * col) as usize;
        let mut dsu = Dsu::new(n + 2);
        let mut m = vec![vec![false; col as usize]; row as usize];
        for (i, c) in cells.iter().enumerate().rev() {
            let (x, y) = (c[0] - 1, c[1] - 1);
            m[x as usize][y as usize] = true;
            let z = (x * col + y) as usize;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (x1, y1) = (x + dx, y + dy);
                if x1 >= 0 && y1 >= 0 && x1 < row && y1 < col && m[x1 as usize][y1 as usize] {
                    dsu.union(z, (x1 * col + y1) as usize);
                }
            }
            if x == 0 {
                dsu.union(z, n);
            }
            if x == row - 1 {
                dsu.union(z, n + 1);
            }
            if dsu.find(n) == dsu.find(n + 1) {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let cells = [[1, 1], [2, 1], [1, 2], [2, 2]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(2, Solution::latest_day_to_cross(2, 2, cells));
    }

    #[test]
    fn case2() {
        let cells = [[1, 1], [1, 2], [2, 1], [2, 2]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(1, Solution::latest_day_to_cross(2, 2, cells));
    }

    #[test]
    fn case3() {
        let cells = [
            [1, 2],
            [2, 1],
            [3, 3],
            [2, 2],
            [1, 1],
            [1, 3],
            [2, 3],
            [3, 2],
            [3, 1],
        ]
        .iter()
        .map(|c| c.to_vec())
        .collect();
        assert_eq!(3, Solution::latest_day_to_cross(3, 3, cells));
    }
}
