use std::collections::VecDeque;

#[derive(Default)]
pub struct FrontMiddleBackQueue {
    f: VecDeque<i32>,
    b: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_front(&mut self, val: i32) {
        self.f.push_front(val);
        if self.f.len() > self.b.len() + 1 {
            let v = self.f.pop_back().unwrap();
            self.b.push_front(v);
        }
    }

    pub fn push_middle(&mut self, val: i32) {
        if self.f.len() > self.b.len() {
            let v = self.f.pop_back().unwrap();
            self.b.push_front(v);
        }
        self.f.push_back(val);
    }

    pub fn push_back(&mut self, val: i32) {
        self.b.push_back(val);
        if self.b.len() > self.f.len() {
            let v = self.b.pop_front().unwrap();
            self.f.push_back(v);
        }
    }

    pub fn pop_front(&mut self) -> i32 {
        let r = self.f.pop_front().unwrap_or(-1);
        if self.f.len() < self.b.len() {
            let v = self.b.pop_front().unwrap();
            self.f.push_back(v);
        }
        r
    }

    pub fn pop_middle(&mut self) -> i32 {
        let r = self.f.pop_back().unwrap_or(-1);
        if self.f.len() < self.b.len() {
            let v = self.b.pop_front().unwrap();
            self.f.push_back(v);
        }
        r
    }

    pub fn pop_back(&mut self) -> i32 {
        let r = self
            .b
            .pop_back()
            .unwrap_or_else(|| self.f.pop_back().unwrap_or(-1));
        if self.f.len() > self.b.len() + 1 {
            let v = self.f.pop_back().unwrap();
            self.b.push_front(v);
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1);
        q.push_back(2);
        q.push_middle(3);
        q.push_middle(4);
        assert_eq!(1, q.pop_front());
        assert_eq!(3, q.pop_middle());
        assert_eq!(4, q.pop_middle());
        assert_eq!(2, q.pop_back());
        assert_eq!(-1, q.pop_front());
    }
}
