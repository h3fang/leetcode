use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn dfs(head: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
            match head {
                Some(mut node) => {
                    let next = node.next.take();
                    let is_none = next.is_none();
                    let (sum, tail) = dfs(next);
                    if node.val == 0 && !is_none {
                        (
                            0,
                            Some(Box::new(ListNode {
                                val: sum,
                                next: tail,
                            })),
                        )
                    } else {
                        (sum + node.val, tail)
                    }
                }
                None => (0, None),
            }
        }
        dfs(head).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[0, 3, 1, 0, 4, 5, 2, 0]);
        let expected = ListNode::from_vec(&[4, 11]);
        assert_eq!(expected, Solution::merge_nodes(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[0, 1, 0, 3, 0, 2, 2, 0]);
        let expected = ListNode::from_vec(&[1, 3, 4]);
        assert_eq!(expected, Solution::merge_nodes(head));
    }
}
