use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut r = &head;
        let mut h = &head;
        let mut two = true;
        while let Some(node) = h {
            two = !two;
            h = &node.next;
            if two {
                if let Some(node) = r {
                    r = &node.next;
                }
            }
        }
        r.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        let result = Solution::middle_node(head);
        assert_eq!(3, result.unwrap().val);
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&vec![1, 2, 3, 4, 5, 6]);
        let result = Solution::middle_node(head);
        assert_eq!(4, result.unwrap().val);
    }
}
