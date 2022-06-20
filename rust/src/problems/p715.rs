const N: i32 = 10_0000_0000;

#[derive(Default)]
struct Node {
    covered: bool,
    lazy: i8,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn push_down(&mut self) {
        let left = self.left.get_or_insert_with(|| Box::new(Node::default()));
        if self.lazy != 0 {
            left.covered = self.lazy == 1;
            left.lazy = self.lazy;
        }
        let right = self.right.get_or_insert_with(|| Box::new(Node::default()));
        if self.lazy != 0 {
            right.covered = self.lazy == 1;
            right.lazy = self.lazy;
            self.lazy = 0;
        }
    }

    fn push_up(&mut self) {
        self.covered = self.left.as_ref().unwrap().covered && self.right.as_ref().unwrap().covered;
    }

    fn query(&mut self, l: i32, r: i32, s: i32, t: i32) -> bool {
        if l <= s && t <= r {
            return self.covered;
        }
        let mid = s + (t - s) / 2;
        let mut result = true;
        self.push_down();
        if l <= mid {
            result &= self.left.as_mut().unwrap().query(l, r, s, mid);
        }
        if r > mid {
            result &= self.right.as_mut().unwrap().query(l, r, mid + 1, t);
        }
        result
    }

    fn update(&mut self, l: i32, r: i32, s: i32, t: i32, cover: i8) {
        if l <= s && r >= t {
            self.covered = cover == 1;
            self.lazy = cover;
            return;
        }
        let mid = s + (t - s) / 2;
        self.push_down();
        if l <= mid {
            self.left.as_mut().unwrap().update(l, r, s, mid, cover);
        }
        if r > mid {
            self.right.as_mut().unwrap().update(l, r, mid + 1, t, cover);
        }
        self.push_up();
    }
}

#[derive(Default)]
pub struct RangeModule {
    root: Node,
}

impl RangeModule {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_range(&mut self, left: i32, right: i32) {
        self.root.update(left, right - 1, 1, N, 1);
    }

    pub fn query_range(&mut self, left: i32, right: i32) -> bool {
        self.root.query(left, right - 1, 1, N)
    }

    pub fn remove_range(&mut self, left: i32, right: i32) {
        self.root.update(left, right - 1, 1, N, -1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut rm = RangeModule::new();
        rm.add_range(10, 20);
        rm.remove_range(14, 16);
        assert_eq!(true, rm.query_range(10, 14));
        assert_eq!(false, rm.query_range(13, 15));
        assert_eq!(true, rm.query_range(16, 17));
    }
}
