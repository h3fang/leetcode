pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<i32>,
    sets: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            sets: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return false;
        }
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Equal => {
                self.rank[px] += 1;
                self.parent[py] = px;
            }
            std::cmp::Ordering::Greater => self.parent[py] = px,
        }
        self.sets -= 1;
        true
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for e in &mut edges {
            e[1] -= 1;
            e[2] -= 1;
        }
        let mut ua = Dsu::new(n as usize);
        let mut ub = Dsu::new(n as usize);
        for e in &edges {
            if e[0] == 3 {
                if !ua.union(e[1] as usize, e[2] as usize) {
                    result += 1;
                } else {
                    ub.union(e[1] as usize, e[2] as usize);
                }
            }
        }
        for e in &edges {
            if e[0] == 1 {
                if !ua.union(e[1] as usize, e[2] as usize) {
                    result += 1;
                }
            } else if e[0] == 2 && !ub.union(e[1] as usize, e[2] as usize) {
                result += 1;
            }
        }
        if ua.sets != 1 || ub.sets != 1 {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [
            [3, 1, 2],
            [3, 2, 3],
            [1, 1, 3],
            [1, 2, 4],
            [1, 1, 2],
            [2, 3, 4],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        assert_eq!(2, Solution::max_num_edges_to_remove(4, edges));
    }

    #[test]
    fn case2() {
        let edges = [[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(0, Solution::max_num_edges_to_remove(4, edges));
    }

    #[test]
    fn case3() {
        let edges = [[3, 2, 3], [1, 1, 2], [2, 3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(-1, Solution::max_num_edges_to_remove(4, edges));
    }
}
