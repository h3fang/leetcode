pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    pub size: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
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
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let n = graph.len();
        let mut dsu = Dsu::new(n);
        for (i, r) in graph.iter().enumerate() {
            for (j, &e) in r.iter().enumerate() {
                if e == 1 {
                    dsu.union(i, j);
                }
            }
        }
        let mut count = vec![0; n];
        for &x in &initial {
            count[dsu.find(x as usize)] += 1;
        }
        let (mut result, mut max) = (*initial.iter().min().unwrap(), 0);
        for x in initial {
            let px = dsu.find(x as usize);
            if count[px] == 1 {
                match max.cmp(&dsu.size[px]) {
                    std::cmp::Ordering::Equal => {
                        result = result.min(x);
                    }
                    std::cmp::Ordering::Less => {
                        max = dsu.size[px];
                        result = x;
                    }
                    _ => {}
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
        let graph = [[1, 1, 0], [1, 1, 0], [0, 0, 1]]
            .iter()
            .map(|g| g.to_vec())
            .collect();
        assert_eq!(0, Solution::min_malware_spread(graph, vec![0, 1]));
    }

    #[test]
    fn case2() {
        let graph = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
            .iter()
            .map(|g| g.to_vec())
            .collect();
        assert_eq!(0, Solution::min_malware_spread(graph, vec![0, 2]));
    }

    #[test]
    fn case3() {
        let graph = [[1, 1, 1], [1, 1, 1], [1, 1, 1]]
            .iter()
            .map(|g| g.to_vec())
            .collect();
        assert_eq!(1, Solution::min_malware_spread(graph, vec![1, 2]));
    }
}
