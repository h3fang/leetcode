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
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        let g = {
            let mut g = vec![vec![]; n];
            for e in edges {
                g[e[0] as usize].push(e[1]);
                g[e[1] as usize].push(e[0]);
            }
            g
        };
        let mut result = n;
        let mut dsu = Dsu::new(n);
        let mut ids = (0..n).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| vals[i]);
        for x in ids {
            let v = vals[x];
            let px = dsu.find(x);
            for &y in &g[x] {
                let py = dsu.find(y as usize);
                if py == px || vals[py] > v {
                    continue;
                }
                if vals[py] == v {
                    result += dsu.size[px] * dsu.size[py];
                    dsu.size[px] += dsu.size[py];
                }
                dsu.parent[py] = px;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let vals = vec![1, 3, 2, 1, 3];
        let edges = [[0, 1], [0, 2], [2, 3], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(6, Solution::number_of_good_paths(vals, edges));
    }

    #[test]
    fn case2() {
        let vals = vec![1, 1, 2, 2, 3];
        let edges = [[0, 1], [1, 2], [2, 3], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(7, Solution::number_of_good_paths(vals, edges));
    }

    #[test]
    fn case3() {
        let vals = vec![1];
        let edges = Vec::new();
        assert_eq!(1, Solution::number_of_good_paths(vals, edges));
    }
}
