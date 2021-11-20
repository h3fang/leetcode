use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut h = head.as_ref();
        let mut nodes = Vec::new();
        while let Some(n) = h {
            nodes.push(n.val);
            h = n.next.as_ref();
        }
        nodes.reverse();
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 3, 2]);
        assert_eq!(vec![2, 3, 1], Solution::reverse_print(head));
    }
}
