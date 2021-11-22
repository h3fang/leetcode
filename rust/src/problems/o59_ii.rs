use std::collections::VecDeque;

#[derive(Default)]
pub struct MaxQueue {
    q: VecDeque<i32>,
    vals: VecDeque<i32>,
}

impl MaxQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_value(&self) -> i32 {
        *self.q.back().unwrap_or(&(-1))
    }

    pub fn push_back(&mut self, value: i32) {
        while let Some(&f) = self.q.front() {
            if value > f {
                self.q.pop_front();
            } else {
                break;
            }
        }
        self.q.push_front(value);
        self.vals.push_back(value);
    }

    pub fn pop_front(&mut self) -> i32 {
        match self.vals.pop_front() {
            Some(v) => {
                if v == *self.q.back().unwrap() {
                    self.q.pop_back();
                }
                v
            }
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut mq = MaxQueue::new();
        mq.push_back(1);
        mq.push_back(2);
        assert_eq!(2, mq.max_value());
        assert_eq!(1, mq.pop_front());
        assert_eq!(2, mq.max_value());
    }
}
