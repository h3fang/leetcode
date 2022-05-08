use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

#[derive(Default)]
pub struct NestedIterator {
    it: VecDeque<NestedInteger>,
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut s = Self::default();
        for ni in nested_list {
            s.it.push_back(ni);
        }
        s
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        let f = self.it.pop_front().unwrap();
        match f {
            NestedInteger::Int(n) => n,
            NestedInteger::List(nis) => {
                for ni in nis.into_iter().rev() {
                    self.it.push_front(ni);
                }
                self.next()
            }
        }
    }

    pub fn has_next(&self) -> bool {
        fn is_val(ni: &NestedInteger) -> bool {
            match ni {
                NestedInteger::Int(_) => true,
                NestedInteger::List(nis) => nis.iter().any(is_val),
            }
        }
        self.it.iter().any(is_val)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        NestedInteger::{Int, List},
        NestedIterator,
    };

    #[test]
    fn case1() {
        let ni = vec![
            List(vec![Int(1), Int(1)]),
            Int(2),
            List(vec![Int(1), Int(1)]),
        ];
        let mut ni = NestedIterator::new(ni);
        let mut result = vec![];
        while ni.has_next() {
            result.push(ni.next());
        }
        assert_eq!(vec![1, 1, 2, 1, 1], result);
    }
}
