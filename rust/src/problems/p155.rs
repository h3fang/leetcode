pub struct MinStack {
    arr: Vec<i64>,
    min: i32,
}

impl MinStack {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            arr: Vec::new(),
            min: i32::MAX,
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.arr.is_empty() {
            self.arr.push(0);
            self.min = val;
        } else {
            self.arr.push(val as i64 - self.min as i64);
            self.min = self.min.min(val);
        }
    }

    pub fn pop(&mut self) {
        let v = self.arr.pop().unwrap();
        if v < 0 {
            self.min = (self.min as i64 - v) as i32;
        }
    }

    pub fn top(&self) -> i32 {
        let top = *self.arr.last().unwrap();
        if top < 0 {
            self.min
        } else {
            (self.min as i64 + top) as i32
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(-3, s.get_min());
        s.pop();
        assert_eq!(0, s.top());
        assert_eq!(-2, s.get_min());
    }

    #[test]
    fn case2() {
        let mut s = MinStack::new();
        s.push(2147483646);
        s.push(2147483646);
        s.push(2147483647);
        assert_eq!(2147483647, s.top());
        s.pop();
        assert_eq!(2147483646, s.get_min());
        s.pop();
        assert_eq!(2147483646, s.get_min());
        s.pop();
        s.push(2147483647);
        assert_eq!(2147483647, s.top());
        assert_eq!(2147483647, s.get_min());
        s.push(-2147483648);
        assert_eq!(-2147483648, s.top());
        assert_eq!(-2147483648, s.get_min());
        s.pop();
        assert_eq!(2147483647, s.get_min());
    }
}
