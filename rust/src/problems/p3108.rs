pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<u32>,
    weight: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            weight: vec![i32::MAX; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn uion(&mut self, x: usize, y: usize, w: i32) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            self.weight[px] &= w;
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.size[py] += self.size[px];
                self.weight[py] &= self.weight[px] & w;
                self.parent[px] = py;
            }
            _ => {
                self.size[px] += self.size[py];
                self.weight[px] &= self.weight[py] & w;
                self.parent[py] = px;
            }
        }
    }
}

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = Dsu::new(n as usize);
        for e in &edges {
            dsu.uion(e[0] as usize, e[1] as usize, e[2]);
        }
        query
            .into_iter()
            .map(|q| {
                let px = dsu.find(q[0] as usize);
                let py = dsu.find(q[1] as usize);
                if px != py { -1 } else { dsu.weight[px] }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1, 7], [1, 3, 7], [1, 2, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let query = [[0, 3], [3, 4]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![1, -1], Solution::minimum_cost(5, edges, query));
    }

    #[test]
    fn case2() {
        let edges = [[0, 2, 7], [0, 1, 15], [1, 2, 6], [1, 2, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let query = [[1, 2]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(vec![0], Solution::minimum_cost(3, edges, query));
    }
}
