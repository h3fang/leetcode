use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn f(head: Option<Box<ListNode>>, right: &mut i32) -> Option<Box<ListNode>> {
            match head {
                Some(mut node) => {
                    let next = f(node.next.take(), right);
                    if node.val < *right {
                        next
                    } else {
                        *right = node.val;
                        Some(Box::new(ListNode {
                            val: node.val,
                            next,
                        }))
                    }
                }
                None => None,
            }
        }
        let mut right = i32::MIN;
        f(head, &mut right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[5, 2, 13, 3, 8]);
        assert_eq!(ListNode::from_vec(&[13, 8]), Solution::remove_nodes(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 1, 1, 1]);
        assert_eq!(
            ListNode::from_vec(&[1, 1, 1, 1]),
            Solution::remove_nodes(head)
        );
    }
}
