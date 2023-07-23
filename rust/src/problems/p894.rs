pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut f = vec![vec![]; n as usize + 1];
        fn apf(
            n: i32,
            f: &mut [Vec<Option<Rc<RefCell<TreeNode>>>>],
        ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if n % 2 == 0 {
                return vec![];
            }
            if n == 1 {
                return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
            }
            if !f[n as usize].is_empty() {
                return f[n as usize].to_vec();
            }

            for i in (1..n).step_by(2) {
                let left = apf(i, f);
                let right = apf(n - 1 - i, f);
                for a in left {
                    for b in &right {
                        let node = TreeNode {
                            val: 0,
                            left: a.clone(),
                            right: b.clone(),
                        };
                        f[n as usize].push(Some(Rc::new(RefCell::new(node))));
                    }
                }
            }

            f[n as usize].to_vec()
        }
        apf(n, &mut f)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let result = Solution::all_possible_fbt(7);
        let trees = vec![
            vec![0, 0, 0, null, null, 0, 0, null, null, 0, 0],
            vec![0, 0, 0, null, null, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, null, null, null, null, 0, 0],
            vec![0, 0, 0, 0, 0, null, null, 0, 0],
        ];
        assert!(result.len() == trees.len());
        for t in trees {
            let root = TreeNode::from_vec(&t);
            assert!(result.contains(&root));
        }
    }
}
