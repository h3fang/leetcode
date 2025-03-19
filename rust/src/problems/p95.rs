use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(lb: i32, ub: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if lb == ub {
                return vec![None];
            }
            let mut result = Vec::new();
            for i in lb..ub {
                let rs = dfs(i + 1, ub);
                for left in dfs(lb, i) {
                    for right in &rs {
                        let node = TreeNode {
                            val: i,
                            left: left.clone(),
                            right: right.clone(),
                        };
                        result.push(Some(Rc::new(RefCell::new(node))));
                    }
                }
            }
            result
        }

        dfs(1, n + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::{LeetCodeTreeNodes, null};

    use super::*;

    #[test]
    fn case1() {
        let trees = Solution::generate_trees(3);
        let mut trees = trees
            .iter()
            .map(|t| t.as_ref().unwrap().borrow().to_string())
            .collect::<Vec<_>>();
        trees.sort_unstable();
        let expected = [
            vec![1, null, 2, null, 3],
            vec![1, null, 3, 2],
            vec![2, 1, 3],
            vec![3, 1, null, null, 2],
            vec![3, 2, null, 1],
        ];
        let mut expected = expected
            .iter()
            .map(|t| LeetCodeTreeNodes { nums: t.clone() }.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, trees);
    }
}
