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
                self.size[py] += self.size[px];
                self.parent[px] = py;
            }
            _ => {
                self.size[px] += self.size[py];
                self.parent[py] = px;
            }
        }
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dsu = Dsu::new(n);
        for e in &edges {
            dsu.union(e[0] as usize, e[1] as usize);
        }
        let mut edges_of_comp = vec![0; n];
        for e in &edges {
            let p = dsu.find(e[0] as usize);
            edges_of_comp[p] += 1;
        }
        let mut ans = 0;
        for (x, edges) in edges_of_comp.into_iter().enumerate() {
            let px = dsu.find(x);
            if x != px {
                continue;
            }
            let size = dsu.size[x] as i32;
            if edges == size * (size - 1) / 2 {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [0, 2], [1, 2], [3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(3, Solution::count_complete_components(6, edges));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(1, Solution::count_complete_components(6, edges));
    }
}
