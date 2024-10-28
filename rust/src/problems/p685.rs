pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
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
        self.parent[px] = py;
    }
}

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut dsu = Dsu::new(n + 1);
        let mut parent: Vec<i32> = (0..=n as i32).collect();
        let mut loop_edge = None;
        let mut two_parents = None;
        for e in edges {
            if parent[e[1] as usize] != e[1] {
                two_parents = Some(e.clone());
            } else {
                parent[e[1] as usize] = e[0];
                if loop_edge.is_none() && dsu.find(e[0] as usize) == dsu.find(e[1] as usize) {
                    loop_edge = Some(e);
                } else {
                    dsu.union(e[0] as usize, e[1] as usize);
                }
            }
        }
        if let Some(e) = two_parents {
            if loop_edge.is_some() {
                vec![parent[e[1] as usize], e[1]]
            } else {
                e
            }
        } else {
            loop_edge.unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2], [1, 3], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![2, 3],
            Solution::find_redundant_directed_connection(edges)
        );
    }

    #[test]
    fn case2() {
        let edges = [[1, 2], [2, 3], [3, 4], [4, 1], [1, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![4, 1],
            Solution::find_redundant_directed_connection(edges)
        );
    }
}
