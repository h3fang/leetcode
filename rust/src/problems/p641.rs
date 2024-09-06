pub struct MyCircularDeque {
    arr: Vec<i32>,
    k: usize,
    start: usize,
    end: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        Self {
            arr: vec![0; k as usize + 1],
            k: k as usize + 1,
            start: 0,
            end: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.start = (self.start + self.k - 1) % self.k;
            self.arr[self.start] = value;
            true
        }
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.arr[self.end] = value;
            self.end = (self.end + 1) % self.k;
            true
        }
    }

    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.start = (self.start + 1) % self.k;
            true
        }
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.end = (self.end + self.k - 1) % self.k;
            true
        }
    }

    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.arr[self.start]
        }
    }

    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.arr[(self.end + self.k - 1) % self.k]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn is_full(&self) -> bool {
        self.end + 1 == self.start || (self.start == 0 && self.end == self.k - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut mcd = MyCircularDeque::new(3);
        assert!(mcd.insert_last(1));
        assert!(mcd.insert_last(2));
        assert!(mcd.insert_front(3));
        assert!(!mcd.insert_front(4));
        assert_eq!(2, mcd.get_rear());
        assert!(mcd.is_full());
        assert!(mcd.delete_last());
        assert!(mcd.insert_front(4));
        assert_eq!(4, mcd.get_front());
    }
}
