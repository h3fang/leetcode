use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut nums = vec![];
        while let Some(mut n) = head {
            nums.push(n.val);
            head = n.next.take();
        }
        let n = nums.len();
        (0..n / 2).map(|i| nums[i] + nums[n - 1 - i]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[5, 4, 2, 1]);
        assert_eq!(6, Solution::pair_sum(head));
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[4, 2, 2, 3]);
        assert_eq!(7, Solution::pair_sum(head));
    }

    #[test]
    fn case3() {
        let head = ListNode::from_vec(&[1, 100000]);
        assert_eq!(100001, Solution::pair_sum(head));
    }
}
