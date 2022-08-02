pub struct MyCircularQueue {
    nums: Vec<i32>,
    start: usize,
    len: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            nums: vec![0; k as usize],
            start: 0,
            len: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            let i = (self.start + self.len) % self.nums.len();
            self.nums[i] = value;
            self.len += 1;
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.start = (self.start + 1) % self.nums.len();
            self.len -= 1;
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.nums[self.start]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let i = (self.start + self.len - 1) % self.nums.len();
            self.nums[i]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.nums.len() == self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut q = MyCircularQueue::new(3);
        assert_eq!(true, q.en_queue(1));
        assert_eq!(true, q.en_queue(2));
        assert_eq!(true, q.en_queue(3));
        assert_eq!(false, q.en_queue(4));
        assert_eq!(3, q.rear());
        assert_eq!(true, q.is_full());
        assert_eq!(true, q.de_queue());
        assert_eq!(true, q.en_queue(4));
        assert_eq!(4, q.rear());
    }
}
