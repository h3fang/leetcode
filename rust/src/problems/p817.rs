use crate::utils::linked_list::ListNode;

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let s = nums.into_iter().collect::<HashSet<_>>();
        let mut h = &head;
        let mut result = 0;
        let mut comp = false;
        while let Some(node) = h {
            if s.contains(&node.val) {
                if !comp {
                    comp = true;
                    result += 1;
                }
            } else {
                comp = false;
            }
            h = &node.next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[0, 1, 2, 3]);
        let nums = vec![0, 1, 3];
        assert_eq!(2, Solution::num_components(head, nums));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[0, 1, 2, 3, 4]);
        let nums = vec![0, 3, 1, 4];
        assert_eq!(2, Solution::num_components(head, nums));
    }
}
