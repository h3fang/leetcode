pub struct Solution;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
struct Node {
    val: i32,
    lazy: i32,
}

struct SegmentTree {
    nodes: Vec<Node>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![Node { val: 0, lazy: 0 }; n * 4],
        }
    }

    fn push_down(&mut self, i: usize) {
        let lazy = self.nodes[i].lazy;
        if lazy == 0 {
            return;
        }
        self.nodes[2 * i].lazy = self.nodes[2 * i].lazy.max(lazy);
        self.nodes[2 * i + 1].lazy = self.nodes[2 * i + 1].lazy.max(lazy);
        self.nodes[2 * i].val = self.nodes[2 * i].val.max(self.nodes[2 * i].lazy);
        self.nodes[2 * i + 1].val = self.nodes[2 * i + 1].val.max(self.nodes[2 * i + 1].lazy);
        self.nodes[i].lazy = 0;
    }

    fn update(&mut self, l: i32, r: i32, lb: i32, rb: i32, i: usize, val: i32) {
        if l <= lb && rb <= r {
            self.nodes[i].lazy = val.max(self.nodes[i].lazy);
            self.nodes[i].val = val.max(self.nodes[i].val);
            return;
        }
        let mid = lb + (rb - lb) / 2;
        self.push_down(i);
        if l <= mid {
            self.update(l, r, lb, mid, 2 * i, val);
        }
        if mid < r {
            self.update(l, r, mid + 1, rb, 2 * i + 1, val);
        }
        self.nodes[i].val = self.nodes[2 * i].val.max(self.nodes[2 * i + 1].val);
    }

    fn query(&mut self, l: i32, r: i32, lb: i32, rb: i32, i: usize) -> i32 {
        if l <= lb && rb <= r {
            return self.nodes[i].val;
        }
        let mid = lb + (rb - lb) / 2;
        self.push_down(i);
        let mut result = 0;
        if l <= mid {
            result = result.max(self.query(l, r, lb, mid, 2 * i));
        }
        if mid < r {
            result = result.max(self.query(l, r, mid + 1, rb, 2 * i + 1));
        }
        result
    }
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // discretization
        let mut points = HashSet::new();
        for b in &buildings {
            points.insert(b[0]);
            points.insert(b[1]);
        }
        let mut points = points.into_iter().collect::<Vec<_>>();
        points.sort_unstable();
        let m = points
            .iter()
            .enumerate()
            .map(|(i, p)| (*p, i as i32))
            .collect::<HashMap<_, _>>();

        // build segment tree
        let n = points.len();
        let mut t = SegmentTree::new(n);
        for b in &buildings {
            let l = *m.get(&b[0]).unwrap() + 1;
            let r = *m.get(&b[1]).unwrap();
            t.update(l, r, 1, n as i32, 1, b[2]);
        }

        // query sky line
        let mut result: Vec<Vec<i32>> = vec![];
        for l in 1..n as i32 {
            let h = t.query(l, l, 1, n as i32, 1);
            if result.is_empty() || h != result.last().unwrap()[1] {
                result.push(vec![points[l as usize - 1], h]);
            }
        }
        result.push(vec![*points.last().unwrap(), 0]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let buildings = [
            [2, 9, 10],
            [3, 7, 15],
            [5, 12, 12],
            [15, 20, 10],
            [19, 24, 8],
        ]
        .iter()
        .map(|b| b.to_vec())
        .collect();
        let skyline = [
            [2, 10],
            [3, 15],
            [7, 12],
            [12, 0],
            [15, 10],
            [20, 8],
            [24, 0],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(skyline, Solution::get_skyline(buildings));
    }

    #[test]
    fn case2() {
        let buildings = [[0, 2, 3], [2, 5, 3]].iter().map(|b| b.to_vec()).collect();
        let skyline = [[0, 3], [5, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(skyline, Solution::get_skyline(buildings));
    }
}
