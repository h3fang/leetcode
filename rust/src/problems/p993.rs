use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        fn depth_parent<'a>(
            root: Option<&'a Rc<RefCell<TreeNode>>>,
            parent: Option<&'a Rc<RefCell<TreeNode>>>,
            x: i32,
        ) -> (i32, Option<&'a Rc<RefCell<TreeNode>>>) {
            match root {
                Some(node) => {
                    let n = node.as_ptr();
                    unsafe {
                        if (*n).val == x {
                            (0, parent)
                        } else {
                            let (dx, px) = depth_parent((*n).left.as_ref(), root, x);
                            if dx >= 0 {
                                return (dx + 1, px);
                            }
                            let (dx, px) = depth_parent((*n).right.as_ref(), root, x);
                            if dx < 0 {
                                (-1, None)
                            } else {
                                (dx + 1, px)
                            }
                        }
                    }
                }
                None => (-1, None),
            }
        }

        let (dx, px) = depth_parent(root.as_ref(), None, x);
        let (dy, py) = depth_parent(root.as_ref(), None, y);
        dx == dy && px != py
    }
}
