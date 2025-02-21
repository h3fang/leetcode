use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h = &mut head;
        while let Some(a) = h {
            while let Some(b) = &mut a.next {
                if a.val == b.val {
                    a.next = b.next.take();
                } else {
                    break;
                }
            }
            h = &mut a.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 1, 2]);
        assert_eq!(
            ListNode::from_vec(&[1, 2]),
            Solution::delete_duplicates(head)
        );
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 1, 2, 3, 3]);
        assert_eq!(
            ListNode::from_vec(&[1, 2, 3]),
            Solution::delete_duplicates(head)
        );
    }
}
