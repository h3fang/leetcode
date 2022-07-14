use std::collections::HashSet;

#[derive(Default)]
pub struct SmallestInfiniteSet {
    removed: HashSet<i32>,
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pop_smallest(&mut self) -> i32 {
        for i in 1.. {
            if !self.removed.contains(&i) {
                self.removed.insert(i);
                return i;
            }
        }
        unreachable!()
    }

    pub fn add_back(&mut self, num: i32) {
        self.removed.remove(&num);
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
