use rand::prelude::*;

use crate::utils::linked_list::ListNode;

pub struct Solution {
    nodes: Vec<i32>,
}

impl Solution {
    pub fn new(head: Option<Box<ListNode>>) -> Self {
        let mut nodes = vec![];
        let mut h = head.as_ref();
        while let Some(node) = h {
            nodes.push(node.val);
            h = node.next.as_ref();
        }
        Self { nodes }
    }

    pub fn get_random(&self) -> i32 {
        let i = rand::rng().random_range(0..self.nodes.len());
        self.nodes[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = Solution::new(ListNode::from_vec(&[1, 2, 3]));
        let mut count = [0; 4];
        const N: usize = 10000;
        (0..N).for_each(|_| {
            let n = s.get_random();
            count[n as usize] += 1;
        });
        for c in &count[1..] {
            let p = *c as f64 / N as f64;
            println!("{p}");
            assert!((p - 1.0 / 3.0).abs() < 5e-2);
        }
    }
}
