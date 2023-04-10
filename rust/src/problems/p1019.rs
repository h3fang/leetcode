pub struct Solution;

use crate::utils::linked_list::ListNode;

impl Solution {
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut s: Vec<(usize, i32)> = vec![];
        let mut i = 0;
        while let Some(mut node) = head {
            result.push(0);
            while !s.is_empty() && s.last().unwrap().1 < node.val {
                result[s.last().unwrap().0] = node.val;
                s.pop();
            }
            s.push((i, node.val));
            head = node.next.take();
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[2, 1, 5]);
        assert_eq!(vec![5, 5, 0], Solution::next_larger_nodes(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[2, 7, 4, 3, 5]);
        assert_eq!(vec![7, 0, 5, 5, 0], Solution::next_larger_nodes(head));
    }
}
