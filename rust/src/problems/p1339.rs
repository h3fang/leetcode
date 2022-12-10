pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn get_sum(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    n.val + get_sum(n.left.as_ref()) + get_sum(n.right.as_ref())
                }
                None => 0,
            }
        }

        fn max_p(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32, best: &mut i32) -> i32 {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let s = n.val
                        + max_p(n.left.as_ref(), sum, best)
                        + max_p(n.right.as_ref(), sum, best);
                    if (2 * s - sum).abs() < (2 * *best - sum).abs() {
                        *best = s;
                    }
                    s
                }
                None => 0,
            }
        }
        let sum = get_sum(root.as_ref());
        let mut best = 0;
        max_p(root.as_ref(), sum, &mut best);
        (((sum - best) as i64 * best as i64) % 10_0000_0007) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(110, Solution::max_product(root));
    }

    #[test]
    fn case2() {
        let root = TreeNode::from_vec(&[1, null, 2, 3, 4, null, null, 5, 6]);
        assert_eq!(90, Solution::max_product(root));
    }
}
