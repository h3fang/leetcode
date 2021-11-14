pub struct Solution;

struct DisjointSetUnion {
    parents: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parents[px] = py,
            std::cmp::Ordering::Equal => {
                self.rank[px] += 1;
                self.parents[py] = px;
            }
            std::cmp::Ordering::Greater => self.parents[py] = px,
        }
    }
}

impl Solution {
    pub fn friend_requests(
        n: i32,
        restrictions: Vec<Vec<i32>>,
        requests: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut dsu = DisjointSetUnion::new(n);

        requests
            .iter()
            .map(|req| {
                let px = dsu.find(req[0] as usize);
                let py = dsu.find(req[1] as usize);
                if px == py {
                    true
                } else {
                    let mut valid = true;
                    for r in &restrictions {
                        if (dsu.find(r[0] as usize) == px && dsu.find(r[1] as usize) == py)
                            || (dsu.find(r[1] as usize) == px && dsu.find(r[0] as usize) == py)
                        {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        dsu.union(req[0] as usize, req[1] as usize);
                    }
                    valid
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let restrictions = vec![vec![0, 1]];
        let requests = vec![vec![0, 2], vec![2, 1]];
        assert_eq!(
            vec![true, false],
            Solution::friend_requests(n, restrictions, requests)
        );
    }

    #[test]
    fn case2() {
        let n = 3;
        let restrictions = vec![vec![0, 1]];
        let requests = vec![vec![1, 2], vec![0, 2]];
        assert_eq!(
            vec![true, false],
            Solution::friend_requests(n, restrictions, requests)
        );
    }

    #[test]
    fn case3() {
        let n = 5;
        let restrictions = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let requests = vec![vec![0, 4], vec![1, 2], vec![3, 1], vec![3, 4]];
        assert_eq!(
            vec![true, false, true, false],
            Solution::friend_requests(n, restrictions, requests)
        );
    }
}
