pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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
                self.size[py] += self.size[py];
            }
            _ => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
        }
    }
}

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut dsu = Dsu::new(c + 1);
        for c in connections {
            dsu.union(c[0] as usize, c[1] as usize);
        }
        let mut roots = vec![0; c + 1];
        let mut m: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::with_capacity(c + 1);
        for (x, e) in roots.iter_mut().enumerate().skip(1) {
            let r = dsu.find(x);
            *e = r;
            let e = m.entry(r).or_default();
            e.push(Reverse(x));
        }
        let mut online = vec![true; c + 1];
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            if q[0] == 1 {
                if online[q[1] as usize] {
                    ans.push(q[1]);
                } else {
                    let r = roots[q[1] as usize];
                    let q = m.get_mut(&r).unwrap();
                    while q.peek().is_some_and(|v| !online[v.0]) {
                        q.pop();
                    }
                    if let Some(&Reverse(x)) = q.peek() {
                        ans.push(x as i32);
                    } else {
                        ans.push(-1);
                    }
                }
            } else {
                online[q[1] as usize] = false;
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
        let c = 5;
        let connections = [[1, 2], [2, 3], [3, 4], [4, 5]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        let queries = [[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(
            vec![3, 2, 3],
            Solution::process_queries(c, connections, queries)
        );
    }

    #[test]
    fn case2() {
        let c = 3;
        let connections = vec![];
        let queries = [[1, 1], [2, 1], [1, 1]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(
            vec![1, -1],
            Solution::process_queries(c, connections, queries)
        );
    }
}
