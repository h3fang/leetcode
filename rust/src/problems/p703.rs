use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let nums = nums.into_iter().map(Reverse).collect::<Vec<_>>();
        let mut heap = BinaryHeap::from(nums);
        while heap.len() > k {
            heap.pop();
        }
        Self { k, heap }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut kthlargest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, kthlargest.add(3));
        assert_eq!(5, kthlargest.add(5));
        assert_eq!(5, kthlargest.add(10));
        assert_eq!(8, kthlargest.add(9));
        assert_eq!(8, kthlargest.add(4));
    }
}
