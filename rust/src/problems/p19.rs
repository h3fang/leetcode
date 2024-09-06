use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut len = 0;
        let mut h = &head;
        while let Some(node) = h {
            h = &node.next;
            len += 1;
        }

        if len == n {
            return head.unwrap().next;
        }

        let mut h = &mut head;
        while let Some(node) = h {
            if len == n + 1 {
                if let Some(node_next) = &mut node.next {
                    let next = node_next.next.take();
                    node.next = next;
                    break;
                }
            }
            h = &mut node.next;
            len -= 1
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::remove_nth_from_end(head, 2);
        let expected = ListNode::from_vec(&[1, 2, 3, 5]).unwrap().to_string();
        assert_eq!(expected, result.unwrap().to_string());
    }
}
