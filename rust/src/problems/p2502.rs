use std::collections::{BTreeMap, HashMap};

pub struct Allocator {
    blocks: BTreeMap<i32, i32>,
    ids: HashMap<i32, (i32, Vec<i32>)>,
}

impl Allocator {
    pub fn new(n: i32) -> Self {
        Self {
            blocks: [(-1, -1), (n, n)].into_iter().collect(),
            ids: Default::default(),
        }
    }

    pub fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let first = self.blocks.first_key_value().unwrap();
        let mut prev = (*first.0, *first.1);
        for (&a, &b) in self.blocks.iter().skip(1) {
            let len = a - prev.1 - 1;
            if len >= size {
                self.blocks.insert(prev.1 + 1, prev.1 + size);
                let e = self.ids.entry(m_id).or_default();
                e.0 += size;
                e.1.push(prev.1 + 1);
                return prev.1 + 1;
            }
            prev = (a, b);
        }
        -1
    }

    pub fn free_memory(&mut self, m_id: i32) -> i32 {
        self.ids
            .remove(&m_id)
            .map(|(size, pos)| {
                pos.iter().for_each(|e| {
                    self.blocks.remove(e);
                });
                size
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut a = Allocator::new(10);
        assert_eq!(0, a.allocate(1, 1));
        assert_eq!(1, a.allocate(1, 2));
        assert_eq!(2, a.allocate(1, 3));
        assert_eq!(1, a.free_memory(2));
        assert_eq!(3, a.allocate(3, 4));
        assert_eq!(1, a.allocate(1, 1));
        assert_eq!(6, a.allocate(1, 1));
        assert_eq!(3, a.free_memory(1));
        assert_eq!(-1, a.allocate(10, 2));
        assert_eq!(0, a.free_memory(7));
    }
}
