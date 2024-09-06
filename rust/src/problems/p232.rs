#[derive(Default)]
pub struct MyQueue {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, x: i32) {
        self.left.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.right.is_empty() {
            while let Some(n) = self.left.pop() {
                self.right.push(n);
            }
        }
        self.right.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        if self.right.is_empty() {
            while let Some(n) = self.left.pop() {
                self.right.push(n);
            }
        }
        *self.right.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(1, q.peek());
        assert_eq!(1, q.pop());
        assert!(!q.empty());
    }
}
