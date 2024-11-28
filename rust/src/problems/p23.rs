use crate::utils::linked_list::ListNode;

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(PartialEq, Eq)]
struct List<'a> {
    node: &'a ListNode,
}

impl PartialOrd for List<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.node.val.cmp(&other.node.val)
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut q = BinaryHeap::new();
        for node in lists.iter().flatten() {
            q.push(Reverse(List { node }));
        }
        let mut nums = vec![];
        while let Some(Reverse(list)) = q.pop() {
            nums.push(list.node.val);
            if let Some(node) = &list.node.next {
                q.push(Reverse(List { node }));
            }
        }
        fn build_list(nums: &[i32]) -> Option<Box<ListNode>> {
            if let Some(&n) = nums.first() {
                Some(Box::new(ListNode {
                    val: n,
                    next: build_list(&nums[1..]),
                }))
            } else {
                None
            }
        }
        build_list(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let lists = [vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let lists = lists.iter().map(|l| ListNode::from_vec(l)).collect();
        let result = Solution::merge_k_lists(lists);
        let expected = ListNode::from_vec(&[1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(expected, result);
    }
}
