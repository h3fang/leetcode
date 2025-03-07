pub struct Solution;

use std::collections::HashSet;

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
        let init = initial.iter().map(|&i| i as usize).collect::<HashSet<_>>();
        let mut dsu = Dsu::new(n);
        for (i, r) in graph.iter().enumerate() {
            if init.contains(&i) {
                continue;
            }
            for (j, &c) in r.iter().enumerate().skip(i + 1) {
                if c == 1 && !init.contains(&j) {
                    dsu.union(i, j);
                }
            }
        }
        let mut roots: Vec<HashSet<usize>> = vec![HashSet::default(); n];
        let mut connected = vec![0; n];
        for &i in &initial {
            for (j, &c) in graph[i as usize].iter().enumerate() {
                if c == 1 && !init.contains(&j) {
                    roots[i as usize].insert(dsu.find(j));
                }
            }
            for &r in &roots[i as usize] {
                connected[r] += 1;
            }
        }
        let (mut result, mut max) = (n as i32, 0);
        for &i in &initial {
            let s: usize = roots[i as usize]
                .iter()
                .map(|&r| if connected[r] == 1 { dsu.size[r] } else { 0 })
                .sum();
            match s.cmp(&max) {
                std::cmp::Ordering::Equal => {
                    result = result.min(i);
                }
                std::cmp::Ordering::Greater => {
                    max = s;
                    result = i;
                }
                _ => {}
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
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(0, Solution::min_malware_spread(graph, vec![0, 1]));
    }

    #[test]
    fn case2() {
        let graph = [[1, 1, 0], [1, 1, 1], [0, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::min_malware_spread(graph, vec![0, 1]));
    }

    #[test]
    fn case3() {
        let graph = [[1, 1, 0, 0], [1, 1, 1, 0], [0, 1, 1, 1], [0, 0, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::min_malware_spread(graph, vec![0, 1]));
    }
}
