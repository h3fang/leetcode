pub struct Solution;

use crate::utils::tree::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_duplicate_subtrees(root: Node) -> Vec<Node> {
        let mut m: HashMap<(i32, i32, i32), (i32, Node)> = HashMap::new();
        let mut set = HashMap::new();
        fn dfs(
            root: Option<&Rc<RefCell<TreeNode>>>,
            m: &mut HashMap<(i32, i32, i32), (i32, Node)>,
            set: &mut HashMap<(i32, i32, i32), Node>,
            idx: &mut i32,
        ) -> i32 {
            if let Some(node) = root {
                let n = node.borrow();
                let l = dfs(n.left.as_ref(), m, set, idx);
                let r = dfs(n.right.as_ref(), m, set, idx);
                let id = (n.val, l, r);
                if let Some((idx, n)) = m.get(&id) {
                    set.entry(id).or_insert_with(|| n.clone());
                    *idx
                } else {
                    *idx += 1;
                    m.insert(id, (*idx, root.cloned()));
                    *idx
                }
            } else {
                0
            }
        }
        dfs(root.as_ref(), &mut m, &mut set, &mut 0);
        set.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tree::null;

    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::from_vec(&[1, 2, 3, 4, null, 2, 4, null, null, 4]);
        let expected = [TreeNode::from_vec(&[2, 4]), TreeNode::from_vec(&[4])];
        let result = Solution::find_duplicate_subtrees(root);
        assert_eq!(expected.len(), result.len());
        for r in result {
            assert!(expected.contains(&r));
        }
    }
}
