pub struct Solution;

use std::collections::{BTreeMap, VecDeque};

#[derive(Default)]
struct MultiSet {
    size: usize,
    s: BTreeMap<i32, i32>,
}

impl MultiSet {
    fn len(&self) -> usize {
        self.size
    }

    fn insert(&mut self, v: i32) {
        *self.s.entry(v).or_default() += 1;
        self.size += 1;
    }

    fn remove(&mut self, v: i32) {
        let e = self.s.get_mut(&v).unwrap();
        *e -= 1;
        self.size -= 1;
        if *e == 0 {
            self.s.remove(&v);
        }
    }

    fn first(&self) -> i32 {
        *self.s.keys().next().unwrap()
    }

    fn last(&self) -> i32 {
        *self.s.keys().last().unwrap()
    }

    fn contains_key(&self, key: &i32) -> bool {
        self.s.contains_key(key)
    }
}

#[derive(Default)]
pub struct MKAverage {
    m: usize,
    k: usize,
    sum: i64,
    q: VecDeque<i32>,
    s0: MultiSet,
    s1: MultiSet,
    s2: MultiSet,
}

impl MKAverage {
    pub fn new(m: i32, k: i32) -> Self {
        Self {
            m: m as usize,
            k: k as usize,
            ..Default::default()
        }
    }

    pub fn add_element(&mut self, num: i32) {
        self.q.push_back(num);

        if self.q.len() <= self.m {
            self.sum += num as i64;
            self.s1.insert(num);
            if self.q.len() == self.m {
                while self.s0.len() < self.k {
                    let v = self.s1.first();
                    self.s0.insert(v);
                    self.s1.remove(v);
                    self.sum -= v as i64;
                }
                while self.s2.len() < self.k {
                    let v = self.s1.last();
                    self.s2.insert(v);
                    self.s1.remove(v);
                    self.sum -= v as i64;
                }
            }
        } else {
            if num < self.s0.last() {
                let v = self.s0.last();
                self.s0.insert(num);
                self.s0.remove(v);
                self.s1.insert(v);
                self.sum += v as i64;
            } else if num > self.s2.first() {
                let v = self.s2.first();
                self.s2.insert(num);
                self.s2.remove(v);
                self.s1.insert(v);
                self.sum += v as i64;
            } else {
                self.s1.insert(num);
                self.sum += num as i64;
            }

            let x = self.q.pop_front().unwrap();
            if self.s0.contains_key(&x) {
                self.s0.remove(x);
                let v = self.s1.first();
                self.s1.remove(v);
                self.s0.insert(v);
                self.sum -= v as i64;
            } else if self.s2.contains_key(&x) {
                self.s2.remove(x);
                let v = self.s1.last();
                self.s1.remove(v);
                self.s2.insert(v);
                self.sum -= v as i64;
            } else {
                self.s1.remove(x);
                self.sum -= x as i64;
            }
        }
    }

    pub fn calculate_mk_average(&self) -> i32 {
        if self.q.len() < self.m {
            -1
        } else {
            (self.sum / (self.m - 2 * self.k) as i64) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut mkv = MKAverage::new(3, 1);
        mkv.add_element(3);
        mkv.add_element(1);
        assert_eq!(-1, mkv.calculate_mk_average());
        mkv.add_element(10);
        assert_eq!(3, mkv.calculate_mk_average());
        mkv.add_element(5);
        mkv.add_element(5);
        mkv.add_element(5);
        assert_eq!(5, mkv.calculate_mk_average());
    }

    #[test]
    fn case2() {
        let mut mkv = MKAverage::new(3, 1);
        mkv.add_element(17612);
        mkv.add_element(74607);
        assert_eq!(-1, mkv.calculate_mk_average());
        mkv.add_element(8272);
        mkv.add_element(33433);
        assert_eq!(33433, mkv.calculate_mk_average());
        mkv.add_element(15456);
        mkv.add_element(64938);
        assert_eq!(33433, mkv.calculate_mk_average());
        mkv.add_element(99741);
    }
}
