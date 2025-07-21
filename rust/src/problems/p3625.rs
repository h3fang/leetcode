pub struct Solution;

use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Fraction {
    x: i32,
    y: i32,
}

impl Fraction {
    // all points are pairwise distinct => (x, y) != (0, 0)
    fn new((mut x, mut y): (i32, i32)) -> Self {
        let k = gcd(x.abs(), y.abs());
        x /= k;
        y /= k;
        if x < 0 || (x == 0 && y < 0) {
            x = -x;
            y = -y;
        }
        Self { x, y }
    }
}

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let cap = (n * n).div_ceil(2);
        let mut f1: HashMap<Fraction, Vec<i32>> = HashMap::with_capacity(cap);
        let mut f2: HashMap<(i32, i32), Vec<Fraction>> = HashMap::with_capacity(cap);
        for (i, p) in points.iter().enumerate() {
            let (x1, y1) = (p[0], p[1]);
            for p in points[..i].iter() {
                let (x2, y2) = (p[0], p[1]);
                let (dx, dy) = (x1 - x2, y1 - y2);
                let k = Fraction::new((dx, dy));
                let b = x1 * k.y - y1 * k.x;
                f1.entry(k).or_default().push(b);
                let mid = ((x1 + x2), (y1 + y2));
                f2.entry(mid).or_default().push(k);
            }
        }

        let mut ans = 0;

        let mut f = HashMap::with_capacity(n);
        for vs in f1.into_values() {
            if vs.len() == 1 {
                continue;
            }
            f.clear();
            for v in vs {
                *f.entry(v).or_insert(0) += 1;
            }
            let mut s = 0;
            for v in f.values() {
                ans += s * v;
                s += v;
            }
        }

        let mut f = HashMap::with_capacity(n);
        for vs in f2.into_values() {
            if vs.len() == 1 {
                continue;
            }
            f.clear();
            for v in vs {
                *f.entry(v).or_insert(0) += 1;
            }
            let mut s = 0;
            for v in f.values() {
                ans -= s * v;
                s += v;
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
        let points = [[-3, 2], [3, 0], [2, 3], [3, 2], [2, -3]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::count_trapezoids(points));
    }

    #[test]
    fn case2() {
        let points = [[0, 0], [1, 0], [0, 1], [2, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(1, Solution::count_trapezoids(points));
    }
}
