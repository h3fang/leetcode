use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut first_point = -1;
        let mut prev_point = -1;

        let mut h = head.as_ref();
        let mut prev: Option<&Box<ListNode>> = None;

        let mut i = 0i32;
        let mut min = i32::MAX;

        while let Some(n) = h {
            if let Some(prev) = prev {
                if (prev.val > n.val && n.next.is_some() && n.val < n.next.as_ref().unwrap().val)
                    || (prev.val < n.val
                        && n.next.is_some()
                        && n.val > n.next.as_ref().unwrap().val)
                {
                    if first_point == -1 {
                        first_point = i;
                    }
                    if prev_point != -1 {
                        min = min.min(i - prev_point);
                    }
                    prev_point = i;
                }
            }
            prev = Some(n);
            h = n.next.as_ref();
            i += 1;
        }

        let min = if min == i32::MAX { -1 } else { min };
        let max = if prev_point > first_point {
            prev_point - first_point
        } else {
            -1
        };

        vec![min, max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[3, 1]);
        assert_eq!(vec![-1, -1], Solution::nodes_between_critical_points(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[5, 3, 1, 2, 5, 1, 2]);
        assert_eq!(vec![1, 3], Solution::nodes_between_critical_points(head));
    }

    #[test]
    fn case3() {
        let head = ListNode::from_vec(&[1, 3, 2, 2, 3, 2, 2, 2, 7]);
        assert_eq!(vec![3, 3], Solution::nodes_between_critical_points(head));
    }

    #[test]
    fn case4() {
        let head = ListNode::from_vec(&[2, 3, 3, 2]);
        assert_eq!(vec![-1, -1], Solution::nodes_between_critical_points(head));
    }
}
