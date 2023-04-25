use std::collections::BTreeSet;

pub struct SmallestInfiniteSet {
    min: i32,
    added: BTreeSet<i32>,
}

impl SmallestInfiniteSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            min: 1,
            added: Default::default(),
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if let Some(i) = self.added.pop_first() {
            match i.cmp(&self.min) {
                std::cmp::Ordering::Less => i,
                std::cmp::Ordering::Equal => {
                    self.min += 1;
                    i
                }
                std::cmp::Ordering::Greater => {
                    self.min += 1;
                    self.min - 1
                }
            }
        } else {
            self.min += 1;
            self.min - 1
        }
    }

    pub fn add_back(&mut self, num: i32) {
        self.added.insert(num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut sis = SmallestInfiniteSet::new();
        sis.add_back(2);
        assert_eq!(1, sis.pop_smallest());
        assert_eq!(2, sis.pop_smallest());
        assert_eq!(3, sis.pop_smallest());
        sis.add_back(1);
        assert_eq!(1, sis.pop_smallest());
        assert_eq!(4, sis.pop_smallest());
        assert_eq!(5, sis.pop_smallest());
    }
}
