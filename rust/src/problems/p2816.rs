use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn dfs(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
            if let Some(mut node) = head {
                let (next, c) = dfs(node.next.take());
                let v = c + node.val * 2;
                let n = Some(Box::new(ListNode { val: v % 10, next }));
                (n, v / 10)
            } else {
                (None, 0)
            }
        }
        let (head, c) = dfs(head);
        if c == 0 {
            head
        } else {
            Some(Box::new(ListNode { val: c, next: head }))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 8, 9]);
        let expected = ListNode::from_vec(&[3, 7, 8]);
        assert_eq!(expected, Solution::double_it(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[9, 9, 9]);
        let expected = ListNode::from_vec(&[1, 9, 9, 8]);
        assert_eq!(expected, Solution::double_it(head));
    }
}
