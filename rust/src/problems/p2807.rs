use crate::utils::linked_list::ListNode;

pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let tail = Self::insert_greatest_common_divisors(node.next.take());
                let a = node.val;
                if tail.is_some() {
                    let b = tail.as_ref().unwrap().val;
                    node.next = Some(Box::new(ListNode {
                        val: gcd(a, b),
                        next: tail,
                    }));
                }
                Some(node)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[18, 6, 10, 3]);
        let expected = ListNode::from_vec(&[18, 6, 6, 2, 10, 1, 3]);
        assert_eq!(expected, Solution::insert_greatest_common_divisors(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[7]);
        let expected = ListNode::from_vec(&[7]);
        assert_eq!(expected, Solution::insert_greatest_common_divisors(head));
    }
}
