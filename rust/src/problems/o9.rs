#[derive(Default)]
pub struct CQueue {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl CQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append_tail(&mut self, value: i32) {
        self.left.push(value);
    }

    pub fn delete_head(&mut self) -> i32 {
        if self.right.is_empty() {
            std::mem::swap(&mut self.left, &mut self.right);
            self.right.reverse();
        }
        self.right.pop().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut q = CQueue::new();
        q.append_tail(3);
        q.append_tail(2);
        assert_eq!(3, q.delete_head());
        assert_eq!(2, q.delete_head());
        assert_eq!(-1, q.delete_head());
        q.append_tail(1);
        assert_eq!(1, q.delete_head());
        assert_eq!(-1, q.delete_head());
    }
}
