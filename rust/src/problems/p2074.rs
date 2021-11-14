use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nodes = Vec::with_capacity(100000);
        let mut h = head.as_ref();
        while let Some(n) = h {
            nodes.push(n.val);
            h = n.next.as_ref();
        }
        let n = nodes.len();
        let mut seg = 1;
        let mut i = 0;
        loop {
            let end = (i + seg).min(n);
            if (end - i) % 2 == 0 {
                nodes[i..end].reverse();
            }
            i += seg;
            if i >= n {
                break;
            }
            seg += 1;
        }

        fn helper(nodes: &[i32]) -> Option<Box<ListNode>> {
            if nodes.is_empty() {
                None
            } else {
                Some(Box::new(ListNode {
                    val: nodes[0],
                    next: helper(&nodes[1..]),
                }))
            }
        }

        helper(&nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[5, 2, 6, 3, 9, 1, 7, 3, 8, 4]);
        let result = Solution::reverse_even_length_groups(head);
        let expected = ListNode::from_vec(&[5, 6, 2, 3, 9, 1, 4, 8, 3, 7]);
        assert_eq!(expected.unwrap().to_string(), result.unwrap().to_string());
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[0, 4, 2, 1, 3]);
        let result = Solution::reverse_even_length_groups(head);
        let expected = ListNode::from_vec(&[0, 2, 4, 3, 1]);
        assert_eq!(expected.unwrap().to_string(), result.unwrap().to_string());
    }
}
