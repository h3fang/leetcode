use std::collections::VecDeque;

#[derive(Default)]
pub struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, x: i32) {
        let n = self.q.len();
        self.q.push_back(x);
        for _ in 0..n {
            let v = self.q.pop_front().unwrap();
            self.q.push_back(v);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    pub fn top(&mut self) -> i32 {
        *self.q.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut s = MyStack::new();
        s.push(1);
        s.push(2);
        assert_eq!(2, s.top());
        assert_eq!(2, s.pop());
        assert_eq!(1, s.top());
        assert!(!s.empty());
        s.push(3);
        assert_eq!(3, s.pop());
        assert_eq!(1, s.pop());
        assert!(s.empty());
    }
}
