pub struct CustomStack {
    s: Vec<i32>,
    inc: Vec<i32>,
}

impl CustomStack {
    pub fn new(max_size: i32) -> Self {
        Self {
            s: Vec::with_capacity(max_size as usize),
            inc: vec![0; max_size as usize],
        }
    }

    pub fn push(&mut self, x: i32) {
        if self.s.len() < self.s.capacity() {
            self.s.push(x);
        }
    }

    pub fn pop(&mut self) -> i32 {
        match self.s.pop() {
            Some(mut x) => {
                let i = self.s.len();
                x += self.inc[i];
                if i > 0 {
                    self.inc[i - 1] += self.inc[i];
                }
                self.inc[i] = 0;
                x
            }
            None => -1,
        }
    }

    pub fn increment(&mut self, k: i32, val: i32) {
        let k = k.min(self.s.len() as i32) - 1;
        if k >= 0 {
            self.inc[k as usize] += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut s = CustomStack::new(3);
        s.push(1);
        s.push(2);
        assert_eq!(2, s.pop());
        s.push(2);
        s.push(3);
        s.push(4);
        s.increment(5, 100);
        s.increment(2, 100);
        assert_eq!(103, s.pop());
        assert_eq!(202, s.pop());
        assert_eq!(201, s.pop());
        assert_eq!(-1, s.pop());
    }
}
