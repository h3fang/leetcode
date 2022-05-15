struct Node {
    sum: i32,
    lb: i32,
    ub: i32,
    left: i32,
    right: i32,
}

impl Node {
    fn new(lb: i32, ub: i32) -> Self {
        Self {
            sum: 0,
            lb,
            ub,
            left: -1,
            right: -1,
        }
    }
}

#[derive(Default)]
pub struct CountIntervals {
    nodes: Vec<Node>,
    root: usize,
}

impl CountIntervals {
    pub fn new() -> Self {
        let nodes = vec![Node {
            sum: 0,
            lb: 1,
            ub: 10_0000_0000,
            left: -1,
            right: -1,
        }];
        let root = 0;
        Self { nodes, root }
    }

    pub fn add(&mut self, left: i32, right: i32) {
        self.helper(self.root, left, right);
    }

    fn helper(&mut self, node: usize, left: i32, right: i32) {
        let n = &self.nodes[node];
        if n.sum == n.ub - n.lb + 1 {
            return;
        }
        if left <= n.lb && right >= n.ub {
            self.nodes[node].sum = n.ub - n.lb + 1;
            return;
        }
        let mid = (n.lb + n.ub) / 2;
        if self.nodes[node].left == -1 {
            self.nodes[node].left = self.nodes.len() as i32;
            self.nodes.push(Node::new(self.nodes[node].lb, mid));
        }

        if self.nodes[node].right == -1 {
            self.nodes[node].right = self.nodes.len() as i32;
            self.nodes.push(Node::new(mid + 1, self.nodes[node].ub));
        }

        if left <= mid {
            self.helper(self.nodes[node].left as usize, left, right);
        }

        if right > mid {
            self.helper(self.nodes[node].right as usize, left, right);
        }
        self.nodes[node].sum = self.nodes[self.nodes[node].left as usize].sum
            + self.nodes[self.nodes[node].right as usize].sum;
    }

    pub fn count(&self) -> i32 {
        self.nodes[self.root].sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ci = CountIntervals::new();
        ci.add(2, 3);
        ci.add(7, 10);
        assert_eq!(6, ci.count());
        ci.add(5, 8);
        assert_eq!(8, ci.count());
    }

    #[test]
    fn case2() {
        let mut ci = CountIntervals::new();
        ci.add(8, 43);
        ci.add(13, 16);
        ci.add(26, 33);
        ci.add(28, 36);
        ci.add(29, 37);
        assert_eq!(36, ci.count());
    }
}
