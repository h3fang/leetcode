use crate::utils::linked_list::ListNode;

pub struct Solution;

#[allow(clippy::borrowed_box)]
fn length(mut head: Option<&Box<ListNode>>) -> usize {
    let mut result = 0;
    while let Some(n) = head {
        result += 1;
        head = n.next.as_ref();
    }
    result
}

fn cut(mut head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut h = &mut head;
    for _ in 1..n {
        h = &mut h.as_mut().unwrap().next;
    }
    let tail = h.as_mut().and_then(|n| n.next.take());
    (head, tail)
}

impl Solution {
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let n = length(head.as_ref()) as i32;
        let x = n / k;
        let a = n - x * k;
        let mut result = Vec::with_capacity(k as usize);
        for _ in 0..a {
            let (part, rem) = cut(head, x + 1);
            result.push(part);
            head = rem;
        }
        for _ in a..k {
            let (part, rem) = cut(head, x);
            result.push(part);
            head = rem;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::from_vec(&[1, 2, 3]);
        let k = 5;
        let result = Solution::split_list_to_parts(head, k);
        assert!(result.len() == k as usize);
        assert!(result
            .windows(2)
            .all(|w| length(w[0].as_ref()).abs_diff(length(w[1].as_ref())) <= 1))
    }

    #[test]
    fn case2() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let k = 3;
        let result = Solution::split_list_to_parts(head, k);
        assert!(result.len() == k as usize);
        assert!(result
            .windows(2)
            .all(|w| length(w[0].as_ref()).abs_diff(length(w[1].as_ref())) <= 1))
    }
}
