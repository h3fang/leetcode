use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut h = head.as_ref();
        while let Some(node) = h {
            result <<= 1;
            result += node.val;
            h = node.next.as_ref();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 0, 1]);
        assert_eq!(5, Solution::get_decimal_value(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[0, 0]);
        assert_eq!(0, Solution::get_decimal_value(head));
    }

    #[test]
    fn case3() {
        let head = ListNode::from_vec(&[1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(18880, Solution::get_decimal_value(head));
    }
}
