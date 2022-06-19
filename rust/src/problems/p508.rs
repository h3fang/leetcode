pub struct Solution;

use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, m: &mut HashMap<i32, i32>) -> i32 {
            match root {
                Some(node) => {
                    let n = node.borrow();
                    let l = dfs(n.left.as_ref(), m);
                    let r = dfs(n.right.as_ref(), m);
                    let sum = l + r + n.val;
                    *m.entry(sum).or_default() += 1;
                    sum
                }
                None => 0,
            }
        }
        let mut m: HashMap<i32, i32> = HashMap::new();
        dfs(root.as_ref(), &mut m);
        let mut result = Vec::with_capacity(10000);
        let mut max = 0;
        for (k, v) in m {
            match v.cmp(&max) {
                std::cmp::Ordering::Equal => result.push(k),
                std::cmp::Ordering::Greater => {
                    max = v;
                    result.clear();
                    result.push(k);
                }
                _ => {}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[5, 2, -3]);
        let mut expected = vec![2, -3, 4];
        expected.sort_unstable();
        let mut result = Solution::find_frequent_tree_sum(root);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
