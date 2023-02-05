pub struct Solution;

use std::collections::BTreeMap;

#[derive(Default)]
pub struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, value: i32) {
        let l0 = self.intervals.range(..=value).last();
        let l1 = self.intervals.range(value + 1..).next();
        match (l0, l1) {
            (None, None) => {
                self.intervals.insert(value, value);
            }
            (None, Some((&l1, &r1))) => {
                if l1 == value + 1 {
                    self.intervals.remove(&l1);
                    self.intervals.insert(value, r1);
                } else {
                    self.intervals.insert(value, value);
                }
            }
            (Some((&l0, &r0)), None) => {
                if r0 + 1 == value {
                    self.intervals.insert(l0, value);
                } else if r0 < value {
                    self.intervals.insert(value, value);
                }
            }
            (Some((&l0, &r0)), Some((&l1, &r1))) => {
                if r0 + 1 == value && l1 == value + 1 {
                    self.intervals.remove(&l1);
                    self.intervals.insert(l0, r1);
                } else if l1 == value + 1 {
                    self.intervals.remove(&l1);
                    self.intervals.insert(value, r1);
                } else if r0 + 1 == value {
                    self.intervals.insert(l0, value);
                } else if r0 < value {
                    self.intervals.insert(value, value);
                }
            }
        }
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.iter().map(|(&k, &v)| vec![k, v]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut sr = SummaryRanges::new();
        sr.add_num(1);
        assert_eq!(vec![vec![1, 1]], sr.get_intervals());
        sr.add_num(3);
        assert_eq!(vec![vec![1, 1], vec![3, 3]], sr.get_intervals());
        sr.add_num(7);
        assert_eq!(vec![vec![1, 1], vec![3, 3], vec![7, 7]], sr.get_intervals());
        sr.add_num(2);
        assert_eq!(vec![vec![1, 3], vec![7, 7]], sr.get_intervals());
        sr.add_num(6);
        assert_eq!(vec![vec![1, 3], vec![6, 7]], sr.get_intervals());
    }
}
