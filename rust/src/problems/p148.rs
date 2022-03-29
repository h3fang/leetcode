use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn split(
            mut head: Option<Box<ListNode>>,
            n: usize,
        ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut h = &mut head;
            let mut i = 0;
            while i < (n + 1) / 2 {
                h = &mut h.as_mut().unwrap().next;
                i += 1;
            }
            let h2 = h.take();
            (head, h2)
        }

        fn merge(
            mut h1: Option<Box<ListNode>>,
            mut h2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut h = ListNode::new(0);
            let mut tail = &mut h.next;
            while h1.is_some() && h2.is_some() {
                let v1 = h1.as_ref().unwrap().val;
                let v2 = h2.as_ref().unwrap().val;
                if v1 < v2 {
                    let h1_next = h1.as_mut().unwrap().next.take();
                    *tail = h1;
                    h1 = h1_next;
                } else {
                    let h2_next = h2.as_mut().unwrap().next.take();
                    *tail = h2;
                    h2 = h2_next;
                }
                tail = &mut tail.as_mut().unwrap().next;
            }
            if h1.is_some() {
                *tail = h1;
            } else if h2.is_some() {
                *tail = h2;
            }
            h.next
        }

        fn sort(head: Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
            if n <= 1 {
                head
            } else {
                let (h1, h2) = split(head, n);
                let h1 = sort(h1, (n + 1) / 2);
                let h2 = sort(h2, n / 2);
                merge(h1, h2)
            }
        }

        let mut n = 0;
        let mut h = &head;
        while let Some(node) = h {
            n += 1;
            h = &node.next;
        }
        sort(head, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[4, 2, 1, 3]);
        assert_eq!(ListNode::from_vec(&[1, 2, 3, 4]), Solution::sort_list(head));
    }
}
