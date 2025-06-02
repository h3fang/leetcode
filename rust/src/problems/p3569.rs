pub struct Solution;

use std::collections::{BTreeSet, HashMap};

struct Seg {
    tree: Vec<(i32, i32)>,
}

impl Seg {
    fn new(n: usize) -> Self {
        let m = 2 << (64 - (n - 1).leading_zeros());
        let tree = vec![(0, 0); m];
        Self { tree }
    }

    fn push_down(&mut self, i: usize) {
        let v = self.tree[i].1;
        if v == 0 {
            return;
        }
        self.tree[2 * i].0 += v;
        self.tree[2 * i].1 += v;
        self.tree[2 * i + 1].0 += v;
        self.tree[2 * i + 1].1 += v;
        self.tree[i].1 = 0;
    }

    fn query(&mut self, i: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if ql <= l && qr >= r {
            return self.tree[i].0;
        }
        self.push_down(i);
        let m = l + (r - l) / 2;
        if qr <= m {
            return self.query(i * 2, l, m, ql, qr);
        }
        if ql > m {
            return self.query(i * 2 + 1, m + 1, r, ql, qr);
        }
        let a = self.query(i * 2, l, m, ql, qr);
        let b = self.query(i * 2 + 1, m + 1, r, ql, qr);
        a.max(b)
    }

    fn update(&mut self, i: usize, l: usize, r: usize, ql: usize, qr: usize, v: i32) {
        if ql <= l && qr >= r {
            self.tree[i].0 += v;
            self.tree[i].1 += v;
            return;
        }
        self.push_down(i);
        let m = l + (r - l) / 2;
        if ql <= m {
            self.update(i * 2, l, m, ql, qr, v);
        }
        if qr > m {
            self.update(i * 2 + 1, m + 1, r, ql, qr, v);
        }
        self.tree[i].0 = self.tree[i * 2].0.max(self.tree[i * 2 + 1].0);
    }
}

fn primes(max: usize) -> Vec<bool> {
    let mut ans = vec![true; max + 1];
    ans[1] = false;
    for i in 2..=max {
        if ans[i] {
            for j in (i * i..=max).step_by(i) {
                ans[j] = false;
            }
        }
    }
    ans
}

fn update(seg: &mut Seg, s: &BTreeSet<usize>, n: usize, val: i32) {
    if s.len() < 2 {
        return;
    }
    let a = *s.first().unwrap();
    let b = *s.last().unwrap();
    seg.update(1, 0, n - 1, a, b, val);
}

impl Solution {
    pub fn maximum_count(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let p = primes(10_0001);
        let mut m: HashMap<i32, BTreeSet<usize>> = HashMap::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            if p[x as usize] {
                m.entry(x).or_default().insert(i);
            }
        }
        let n = nums.len();
        let mut seg = Seg::new(n);
        for s in m.values() {
            if s.len() >= 2 {
                let a = *s.first().unwrap();
                let b = *s.last().unwrap();
                seg.update(1, 0, n - 1, a, b, 1);
            }
        }
        queries
            .into_iter()
            .map(|q| {
                let (i, v) = (q[0] as usize, q[1]);
                let old = nums[i];
                nums[i] = v;
                if p[old as usize] {
                    if let Some(s) = m.get_mut(&old) {
                        update(&mut seg, s, n, -1);
                        s.remove(&i);
                        update(&mut seg, s, n, 1);
                        if s.is_empty() {
                            m.remove(&old);
                        }
                    }
                }

                if p[v as usize] {
                    let s = m.entry(v).or_default();
                    update(&mut seg, s, n, -1);
                    s.insert(i);
                    update(&mut seg, s, n, 1);
                }
                m.len() as i32 + seg.query(1, 0, n - 1, 0, n - 1)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 1, 3, 1, 2];
        let queries = [[1, 2], [3, 3]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![3, 4], Solution::maximum_count(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![2, 1, 4];
        let queries = [[0, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![0], Solution::maximum_count(nums, queries));
    }
}
