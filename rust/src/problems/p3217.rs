use crate::utils::linked_list::ListNode;

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn dfs(nums: &HashSet<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head {
                Some(mut node) => {
                    let tail = dfs(nums, node.next.take());
                    if nums.contains(&node.val) {
                        tail
                    } else {
                        node.next = tail;
                        Some(node)
                    }
                }
                None => None,
            }
        }
        let nums: HashSet<i32> = nums.into_iter().collect();
        dfs(&nums, head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3];
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(&[4, 5]);
        assert_eq!(expected, Solution::modified_list(nums, head));
    }

    #[test]
    fn case2() {
        let nums = vec![1];
        let head = ListNode::from_vec(&[1, 2, 1, 2, 1, 2]);
        let expected = ListNode::from_vec(&[2, 2, 2]);
        assert_eq!(expected, Solution::modified_list(nums, head));
    }

    #[test]
    fn case3() {
        let nums = vec![5];
        let head = ListNode::from_vec(&[1, 2, 3, 4]);
        let expected = ListNode::from_vec(&[1, 2, 3, 4]);
        assert_eq!(expected, Solution::modified_list(nums, head));
    }
}
