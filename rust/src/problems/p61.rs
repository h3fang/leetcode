use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut h = &head;
        let mut n_nodes = 0i32;
        while let Some(node) = h {
            n_nodes += 1;
            h = &node.next;
        }
        if k % n_nodes == 0 {
            return head;
        }
        let to_right = (n_nodes - k % n_nodes) as usize;
        let mut head = head;
        let mut h = &mut head;
        for _ in 0..to_right {
            if let Some(node) = h {
                h = &mut node.next;
            }
        }
        let mut new_head = h.take();
        let mut h = &mut new_head;
        while let Some(node) = h {
            h = &mut node.next;
        }
        *h = head;
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let rotated = Solution::rotate_right(head, 2);
        assert_eq!(
            rotated.unwrap().to_string(),
            ListNode::from_vec(&[4, 5, 1, 2, 3]).unwrap().to_string()
        );
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[0, 1, 2]);
        let rotated = Solution::rotate_right(head, 4);
        assert_eq!(
            rotated.unwrap().to_string(),
            ListNode::from_vec(&[2, 0, 1]).unwrap().to_string()
        );
    }

    #[test]
    fn case3() {
        let head = ListNode::from_vec(&[]);
        let rotated = Solution::rotate_right(head, 0);
        assert_eq!(rotated, None);
    }

    #[test]
    fn case4() {
        let head = ListNode::from_vec(&[1, 2]);
        let rotated = Solution::rotate_right(head, 0);
        assert_eq!(
            rotated.unwrap().to_string(),
            ListNode::from_vec(&[1, 2]).unwrap().to_string()
        );
    }

    #[test]
    fn case5() {
        let head = ListNode::from_vec(&[1, 2]);
        let rotated = Solution::rotate_right(head, 1);
        assert_eq!(
            rotated.unwrap().to_string(),
            ListNode::from_vec(&[2, 1]).unwrap().to_string()
        );
    }
}
