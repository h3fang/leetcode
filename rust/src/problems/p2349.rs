use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
pub struct NumberContainers {
    i2n: HashMap<i32, i32>,
    n2i: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn change(&mut self, index: i32, number: i32) {
        if let Some(n) = self.i2n.insert(index, number) {
            self.n2i.get_mut(&n).unwrap().remove(&index);
            self.n2i.entry(number).or_default().insert(index);
        } else {
            self.n2i.entry(number).or_default().insert(index);
        }
    }

    pub fn find(&self, number: i32) -> i32 {
        if let Some(s) = self.n2i.get(&number) {
            if let Some(&i) = s.iter().next() {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nc = NumberContainers::new();
        assert_eq!(-1, nc.find(10));
        nc.change(2, 10);
        nc.change(1, 10);
        nc.change(3, 10);
        nc.change(5, 10);
        assert_eq!(1, nc.find(10));
        nc.change(1, 20);
        assert_eq!(2, nc.find(10));
    }
}
