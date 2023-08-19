pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    max_size: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            max_size: 1,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py {
            return false;
        }
        if self.size[px] < self.size[py] {
            // (px, py) = (py, px);
            std::mem::swap(&mut px, &mut py);
        }
        self.parent[py] = px;
        self.size[px] += self.size[py];
        self.max_size = self.max_size.max(self.size[px]);
        true
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ids = (0..edges.len()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| edges[i][2]);

        let mut mst_all = 0;
        let mut dsu = Dsu::new(n);
        for &i in &ids {
            let e = &edges[i];
            if dsu.union(e[0] as usize, e[1] as usize) {
                mst_all += e[2];
            }
        }
        let mut result = vec![vec![]; 2];
        for i in 0..edges.len() {
            let mut mst = 0;
            let mut dsu = Dsu::new(n);
            for &j in &ids {
                let e = &edges[j];
                if i != j && dsu.union(e[0] as usize, e[1] as usize) {
                    mst += e[2];
                }
            }
            if dsu.max_size < n || mst > mst_all {
                result[0].push(i as i32);
            } else {
                let mut mst = edges[i][2];
                let mut dsu = Dsu::new(n);
                dsu.union(edges[i][0] as usize, edges[i][1] as usize);
                for &j in &ids {
                    let e = &edges[j];
                    if i != j && dsu.union(e[0] as usize, e[1] as usize) {
                        mst += e[2];
                    }
                }
                if mst == mst_all {
                    result[1].push(i as i32);
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
        let n = 5;
        let edges = [
            [0, 1, 1],
            [1, 2, 1],
            [2, 3, 2],
            [0, 3, 2],
            [0, 4, 3],
            [3, 4, 3],
            [1, 4, 6],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let mut result = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        result.iter_mut().for_each(|v| v.sort_unstable());
        assert_eq!(vec![vec![0, 1], vec![2, 3, 4, 5]], result);
    }

    #[test]
    fn case2() {
        let n = 4;
        let edges = [[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let mut result = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        result.iter_mut().for_each(|v| v.sort_unstable());
        assert_eq!(vec![vec![], vec![0, 1, 2, 3]], result);
    }
}
