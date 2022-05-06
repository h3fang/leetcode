use std::collections::VecDeque;

#[derive(Default)]
pub struct RecentCounter {
    reqs: VecDeque<i32>,
}

impl RecentCounter {
    pub fn new() -> Self {
        Self {
            reqs: VecDeque::with_capacity(3001),
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.reqs.push_back(t);
        while let Some(r) = self.reqs.front() {
            if t - r <= 3000 {
                break;
            }
            self.reqs.pop_front();
        }
        self.reqs.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut rc = RecentCounter::new();
        assert_eq!(1, rc.ping(1));
        assert_eq!(2, rc.ping(100));
        assert_eq!(3, rc.ping(3001));
        assert_eq!(3, rc.ping(3002));
    }
}
