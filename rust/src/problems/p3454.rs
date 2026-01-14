pub struct Solution;

use std::collections::BTreeSet;

struct SegmentTree {
    xs: Vec<i32>,
    count: Vec<i32>,
    len: Vec<i32>,
}

impl SegmentTree {
    fn new(xs: Vec<i32>) -> Self {
        let n = xs.len() - 1;

        Self {
            xs,
            count: vec![0; 4 * n],
            len: vec![0; 4 * n],
        }
    }

    fn update(&mut self, ql: i32, qr: i32, v: i32) {
        self._update(0, 0, self.xs.len() - 2, ql, qr, v);
    }

    fn _update(&mut self, i: usize, l: usize, r: usize, ql: i32, qr: i32, v: i32) {
        if self.xs[r + 1] <= ql || self.xs[l] >= qr {
            return;
        }
        if ql <= self.xs[l] && self.xs[r + 1] <= qr {
            self.count[i] += v;
        } else {
            let m = (l + r) / 2;
            self._update(2 * i + 1, l, m, ql, qr, v);
            self._update(2 * i + 2, m + 1, r, ql, qr, v);
        }

        if self.count[i] > 0 {
            self.len[i] = self.xs[r + 1] - self.xs[l];
        } else if l == r {
            self.len[i] = 0;
        } else {
            self.len[i] = self.len[2 * i + 1] + self.len[2 * i + 2];
        }
    }
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let n = squares.len();
        let mut xs = BTreeSet::new();
        let mut events = Vec::with_capacity(n * 2);
        for s in squares {
            let (xl, xr, y, l) = (s[0], s[0] + s[2], s[1], s[2]);
            xs.insert(xl);
            xs.insert(xr);
            events.push((y, 1, xl, xr));
            events.push((y + l, -1, xl, xr));
        }
        events.sort_unstable();
        let xs = xs.into_iter().collect::<Vec<_>>();
        let mut t = SegmentTree::new(xs);
        let (mut total, mut prev) = (0, events[0].0);
        let mut sums = Vec::with_capacity(events.len());
        let mut widths = Vec::with_capacity(events.len());
        for &(y, v, xl, xr) in &events {
            total += t.len[0] as i64 * (y - prev) as i64;
            t.update(xl, xr, v);
            sums.push(total);
            widths.push(t.len[0]);
            prev = y;
        }

        let half = (total + 1) / 2;
        let i = sums.iter().position(|&s| s >= half).unwrap() - 1;
        events[i].0 as f64 + (total - sums[i] * 2) as f64 / (widths[i] as f64 * 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {a:.5}, b = {b:.5}");
    }

    #[test]
    fn case1() {
        let squares = [[0, 0, 1], [2, 2, 1]].iter().map(|v| v.to_vec()).collect();
        assert_close(1.0, Solution::separate_squares(squares));
    }

    #[test]
    fn case2() {
        let squares = [[0, 0, 2], [1, 1, 1]].iter().map(|v| v.to_vec()).collect();
        assert_close(1.0, Solution::separate_squares(squares));
    }
}
