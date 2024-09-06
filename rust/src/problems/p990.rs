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
        if px == py {
            return;
        }
        self.parent[px] = py;
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut eq = Dsu::new(26);
        for e in &equations {
            let e = e.as_bytes();
            if e[1] != b'=' {
                continue;
            }
            let x = (e[0] - b'a') as usize;
            let y = (e[3] - b'a') as usize;
            if x == y {
                return false;
            } else {
                eq.union(x, y);
            }
        }
        for e in &equations {
            let e = e.as_bytes();
            if e[1] != b'!' {
                continue;
            }
            let x = (e[0] - b'a') as usize;
            let y = (e[3] - b'a') as usize;
            if x == y || eq.find(x) == eq.find(y) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let equations = ["a==b", "b!=a"].iter().map(|e| e.to_string()).collect();
        assert!(!Solution::equations_possible(equations));
    }

    #[test]
    fn case2() {
        let equations = ["b==a", "a==b"].iter().map(|e| e.to_string()).collect();
        assert!(Solution::equations_possible(equations));
    }
}
