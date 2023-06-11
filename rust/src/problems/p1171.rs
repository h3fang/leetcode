use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nodes = Vec::with_capacity(1000);
        while let Some(mut n) = head {
            nodes.push(n.val);
            head = n.next.take();
        }
        let mut s = Vec::with_capacity(nodes.len());
        for v in nodes {
            s.push(v);
            let mut sum = 0;
            let mut k = 0;
            for &v in s.iter().rev() {
                sum += v;
                k += 1;
                if sum == 0 {
                    s.resize(s.len() - k, 0);
                    break;
                }
            }
        }
        let mut next = None;
        for val in s.into_iter().rev() {
            next = Some(Box::new(ListNode { val, next }));
        }
        next
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, -3, 3, 1]);
        let expected = [ListNode::from_vec(&[3, 1]), ListNode::from_vec(&[1, 2, 1])];
        assert!(expected.contains(&Solution::remove_zero_sum_sublists(head)));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 2, 3, -3, 4]);
        let expected = ListNode::from_vec(&[1, 2, 4]);
        assert_eq!(expected, Solution::remove_zero_sum_sublists(head));
    }

    #[test]
    fn case3() {
        let head = ListNode::from_vec(&[1, 2, 3, -3, -2]);
        let expected = ListNode::from_vec(&[1]);
        assert_eq!(expected, Solution::remove_zero_sum_sublists(head));
    }
}
