use std::collections::HashMap;

pub struct TopVotedCandidate {
    tops: Vec<i32>,
    times: Vec<i32>,
}

impl TopVotedCandidate {
    pub fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut cnt = HashMap::new();
        let mut top = 0;
        let tops = persons
            .iter()
            .map(|&p| {
                let v = cnt.entry(p).or_insert(0);
                *v += 1;
                if *v >= *cnt.get(&top).unwrap_or(&0) {
                    top = p;
                }
                top
            })
            .collect();
        Self { tops, times }
    }

    pub fn q(&self, t: i32) -> i32 {
        let (mut l, mut r) = (0, self.times.len());
        while l < r {
            let m = (l + r) / 2;
            if self.times[m] > t {
                r = m;
            } else {
                l = m + 1;
            }
        }
        self.tops[l - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tvc = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
        assert_eq!(0, tvc.q(3));
        assert_eq!(1, tvc.q(12));
        assert_eq!(1, tvc.q(25));
        assert_eq!(0, tvc.q(15));
        assert_eq!(0, tvc.q(24));
        assert_eq!(1, tvc.q(8));
    }
}
