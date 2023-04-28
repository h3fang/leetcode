pub struct Solution;

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
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
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Equal => {
                self.rank[px] += 1;
                self.parent[py] = px;
            }
            std::cmp::Ordering::Greater => self.parent[py] = px,
        }
    }
}

fn check(s1: &str, s2: &str) -> bool {
    let mut c = 0;
    for (a, b) in s1.as_bytes().iter().zip(s2.as_bytes()) {
        if a != b {
            c += 1;
        }
        if c > 2 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut dsu = Dsu::new(n);
        for i in 0..n {
            for j in i + 1..n {
                if check(&strs[i], &strs[j]) {
                    dsu.union(i, j);
                }
            }
        }
        dsu.parent
            .iter()
            .enumerate()
            .filter(|&(i, &p)| i == p)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = ["tars", "rats", "arts", "star"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(2, Solution::num_similar_groups(strs));
    }

    #[test]
    fn case2() {
        let strs = ["omv", "ovm"].iter().map(|s| s.to_string()).collect();
        assert_eq!(1, Solution::num_similar_groups(strs));
    }
}
