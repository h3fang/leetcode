pub struct Solution;

struct DisjointSetUnion {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSetUnion {
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
        if px != py {
            match self.rank[px].cmp(&self.rank[py]) {
                std::cmp::Ordering::Less => self.parent[px] = self.parent[py],
                std::cmp::Ordering::Equal => {
                    self.parent[px] = self.parent[py];
                    self.rank[py] += 1;
                }
                std::cmp::Ordering::Greater => self.parent[py] = self.parent[px],
            }
        }
    }

    fn isolate(&mut self, x: usize) {
        if x != self.parent[x] {
            self.parent[x] = x;
            self.rank[x] = 0;
        }
    }
}

impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        meetings.sort_unstable_by_key(|m| m[2]);
        let mut dsu = DisjointSetUnion::new(n as usize);
        dsu.union(0, first_person as usize);

        fn propagate(curr: &[&Vec<i32>], dsu: &mut DisjointSetUnion) {
            for &p in curr {
                dsu.union(p[0] as usize, p[1] as usize);
            }

            for &p in curr {
                if dsu.find(p[0] as usize) != dsu.find(0) {
                    dsu.isolate(p[0] as usize);
                }

                if dsu.find(p[1] as usize) != dsu.find(0) {
                    dsu.isolate(p[1] as usize);
                }
            }
        }

        let mut curr: Vec<&Vec<i32>> = Vec::new();
        for m in &meetings {
            if !(curr.is_empty() || m[2] == curr.last().unwrap()[2]) {
                propagate(&curr, &mut dsu);
                curr.clear();
            }
            curr.push(m);
        }
        propagate(&curr, &mut dsu);

        (0..n as usize)
            .filter_map(|i| {
                if dsu.find(i) == dsu.find(0) {
                    Some(i as i32)
                } else {
                    None
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
        let n = 6;
        let meetings = vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]];
        let first_person = 1;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(vec![0, 1, 2, 3, 5], result);
    }

    #[test]
    fn case2() {
        let n = 4;
        let meetings = vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]];
        let first_person = 3;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(vec![0, 1, 3], result);
    }

    #[test]
    fn case3() {
        let n = 5;
        let meetings = vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]];
        let first_person = 3;
        let mut result = Solution::find_all_people(n, meetings, first_person);
        result.sort_unstable();
        assert_eq!(vec![0, 1, 2, 3, 4], result);
    }
}
