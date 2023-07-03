use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn get_num(mut l: Option<Box<ListNode>>) -> Vec<i32> {
            let mut a = vec![];
            while let Some(mut n) = l {
                a.push(n.val);
                l = n.next.take();
            }
            a
        }
        let mut l1 = get_num(l1);
        let mut l2 = get_num(l2);
        let mut c = 0;
        let mut r = None;
        while !l1.is_empty() || !l2.is_empty() || c != 0 {
            let d = c + l1.pop().unwrap_or(0) + l2.pop().unwrap_or(0);
            r = Some(Box::new(ListNode {
                val: d % 10,
                next: r,
            }));
            c = d / 10;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let l1 = ListNode::from_vec(&[7, 2, 4, 3]);
        let l2 = ListNode::from_vec(&[5, 6, 4]);
        let expected = ListNode::from_vec(&[7, 8, 0, 7]);
        assert_eq!(expected, Solution::add_two_numbers(l1, l2));
    }

    #[test]
    fn case2() {
        let l1 = ListNode::from_vec(&[2, 4, 3]);
        let l2 = ListNode::from_vec(&[5, 6, 4]);
        let expected = ListNode::from_vec(&[8, 0, 7]);
        assert_eq!(expected, Solution::add_two_numbers(l1, l2));
    }

    #[test]
    fn case3() {
        let l1 = ListNode::from_vec(&[0]);
        let l2 = ListNode::from_vec(&[0]);
        let expected = ListNode::from_vec(&[0]);
        assert_eq!(expected, Solution::add_two_numbers(l1, l2));
    }
}
