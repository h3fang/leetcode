pub struct Solution;

struct STNode {
    left: Option<Box<STNode>>,
    right: Option<Box<STNode>>,
    l: i32,
    r: i32,
    mid: i32,
    v: i32,
    add: i32,
}

impl STNode {
    fn new(l: i32, r: i32) -> Self {
        Self {
            left: None,
            right: None,
            l,
            r,
            mid: l + (r - l) / 2,
            v: 0,
            add: 0,
        }
    }

    fn pushdown(&mut self) {
        if self.left.is_none() {
            self.left = Some(Box::new(Self::new(self.l, self.mid)));
        }

        if self.right.is_none() {
            self.right = Some(Box::new(Self::new(self.mid + 1, self.r)));
        }

        if self.add > 0 {
            self.left.as_mut().unwrap().v = self.add;
            self.right.as_mut().unwrap().v = self.add;
            self.left.as_mut().unwrap().add = self.add;
            self.right.as_mut().unwrap().add = self.add;
            self.add = 0;
        }
    }

    fn pushup(&mut self) {
        self.v = self
            .left
            .as_ref()
            .unwrap()
            .v
            .max(self.right.as_ref().unwrap().v);
    }
}

struct SegmentTree {
    root: Option<Box<STNode>>,
}

impl SegmentTree {
    fn new() -> Self {
        Self {
            root: Some(Box::new(STNode::new(0, i32::MAX))),
        }
    }

    fn query(&mut self, l: i32, r: i32) -> i32 {
        fn helper(node: Option<&mut Box<STNode>>, l: i32, r: i32) -> i32 {
            if let Some(node) = node {
                if node.l >= l && node.r <= r {
                    return node.v;
                }
                let mut result = 0;
                node.pushdown();
                if l <= node.mid {
                    result = helper(node.left.as_mut(), l, r);
                }
                if node.mid < r {
                    result = result.max(helper(node.right.as_mut(), l, r));
                }
                result
            } else {
                0
            }
        }
        helper(self.root.as_mut(), l, r)
    }

    fn update(&mut self, l: i32, r: i32, v: i32) {
        fn helper(node: Option<&mut Box<STNode>>, l: i32, r: i32, v: i32) {
            if let Some(node) = node {
                if node.l >= l && node.r <= r {
                    node.v = v;
                    node.add = v;
                    return;
                }
                node.pushdown();
                if l <= node.mid {
                    helper(node.left.as_mut(), l, r, v);
                }
                if node.mid < r {
                    helper(node.right.as_mut(), l, r, v);
                }
                node.pushup();
            }
        }
        helper(self.root.as_mut(), l, r, v);
    }
}

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(positions.len());
        let mut st = SegmentTree::new();
        let mut max = 0;
        for p in positions {
            let (left, side) = (p[0], p[1]);
            let right = left + side - 1;
            let h = st.query(left, right) + side;
            st.update(left, right, h);
            max = max.max(h);
            result.push(max);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let positions = vec![vec![1, 2], vec![2, 3], vec![6, 1]];
        assert_eq!(vec![2, 5, 5], Solution::falling_squares(positions));
    }
}
