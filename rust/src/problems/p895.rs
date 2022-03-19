use std::collections::HashMap;

#[derive(Default)]
pub struct FreqStack {
    max_freq: usize,
    freq: HashMap<i32, usize>,
    groups: HashMap<usize, Vec<i32>>,
}

impl FreqStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        let f = self.freq.entry(val).or_insert(0);
        *f += 1;
        if *f > self.max_freq {
            self.max_freq = *f;
        }
        self.groups.entry(*f).or_default().push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let v = self.groups.entry(self.max_freq).or_default().pop().unwrap();
        *self.freq.entry(v).or_default() -= 1;
        if self.groups.entry(self.max_freq).or_default().is_empty() {
            self.max_freq -= 1;
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut fs = FreqStack::new();
        fs.push(5);
        fs.push(7);
        fs.push(5);
        fs.push(7);
        fs.push(4);
        fs.push(5);
        assert_eq!(5, fs.pop());
        assert_eq!(7, fs.pop());
        assert_eq!(5, fs.pop());
        assert_eq!(4, fs.pop());
    }
}
