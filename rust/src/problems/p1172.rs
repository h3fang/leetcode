use std::{cmp::Reverse, collections::BinaryHeap};

pub struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    q: BinaryHeap<Reverse<usize>>,
}

impl DinnerPlates {
    pub fn new(capacity: i32) -> Self {
        let mut q = BinaryHeap::with_capacity(100001);
        q.push(Reverse(0));
        Self {
            capacity: capacity as usize,
            stacks: Vec::with_capacity(100001),
            q,
        }
    }

    pub fn push(&mut self, val: i32) {
        while let Some(Reverse(i)) = self.q.pop() {
            if i >= self.stacks.len() {
                self.q.clear();
                break;
            } else if self.stacks[i].len() == self.capacity {
                continue;
            } else {
                self.stacks[i].push(val);
                if self.stacks[i].len() < self.capacity {
                    self.q.push(Reverse(i));
                }
                return;
            }
        }
        let i = self.stacks.len();
        self.stacks.push(vec![val]);
        if 1 < self.capacity {
            self.q.push(Reverse(i));
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.pop_at_stack(self.stacks.len() as i32 - 1)
    }

    pub fn pop_at_stack(&mut self, index: i32) -> i32 {
        if index < 0
            || index as usize >= self.stacks.len()
            || self.stacks[index as usize].is_empty()
        {
            -1
        } else {
            let i = index as usize;
            if self.stacks[i].len() == self.capacity {
                self.q.push(Reverse(i));
            }
            let r = self.stacks[i].pop().unwrap();
            while !self.stacks.is_empty() && self.stacks.last().unwrap().is_empty() {
                self.stacks.pop();
            }
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut dp = DinnerPlates::new(2);
        for n in 1..=5 {
            dp.push(n);
        }
        assert_eq!(2, dp.pop_at_stack(0));
        dp.push(20);
        dp.push(21);
        assert_eq!(20, dp.pop_at_stack(0));
        assert_eq!(21, dp.pop_at_stack(2));
        for n in [5, 4, 3, 1, -1] {
            assert_eq!(n, dp.pop());
        }
    }
}
