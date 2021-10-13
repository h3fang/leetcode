use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        #[allow(clippy::borrowed_box)]
        fn helper(l1: Option<&Box<ListNode>>, l2: Option<&Box<ListNode>>) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        Some(Box::new(ListNode {
                            val: n1.val,
                            next: helper(n1.next.as_ref(), l2),
                        }))
                    } else {
                        Some(Box::new(ListNode {
                            val: n2.val,
                            next: helper(l1, n2.next.as_ref()),
                        }))
                    }
                }
                (Some(n1), None) => Some(n1.to_owned()),
                (None, Some(n2)) => Some(n2.to_owned()),
                (None, None) => None,
            }
        }

        helper(l1.as_ref(), l2.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let l1 = ListNode::from_vec(&[1, 2, 4]);
        let l2 = ListNode::from_vec(&[1, 3, 4]);
        let result = Solution::merge_two_lists(l1, l2);
        let expected = ListNode::from_vec(&[1, 1, 2, 3, 4, 4]).unwrap().to_string();
        assert_eq!(expected, result.unwrap().to_string());
    }
}
