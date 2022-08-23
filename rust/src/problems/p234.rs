use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut nodes = Vec::new();
        let mut h = head.as_ref();
        while let Some(n) = h {
            nodes.push(n.val);
            h = n.next.as_ref();
        }
        let n = nodes.len();
        (0..n / 2).all(|i| nodes[i] == nodes[n - 1 - i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 2, 1]);
        assert_eq!(true, Solution::is_palindrome(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 2, 2]);
        assert_eq!(false, Solution::is_palindrome(head));
    }
}
