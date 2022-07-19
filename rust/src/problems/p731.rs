use std::collections::HashMap;

#[derive(Default)]
pub struct MyCalendarTwo {
    tree: HashMap<i32, (i32, i32)>,
}

impl MyCalendarTwo {
    pub fn new() -> Self {
        Default::default()
    }

    fn update(&mut self, start: i32, end: i32, val: i32, l: i32, r: i32, idx: i32) {
        if end < l || start > r {
            return;
        }
        if start <= l && end >= r {
            let e = self.tree.entry(idx).or_default();
            (*e).0 += val;
            (*e).1 += val;
        } else {
            let mid = l + (r - l) / 2;
            self.update(start, end, val, l, mid, 2 * idx);
            self.update(start, end, val, mid + 1, r, 2 * idx + 1);
            let v = self
                .tree
                .entry(2 * idx)
                .or_default()
                .0
                .max(self.tree.entry(2 * idx + 1).or_default().0);
            let e = self.tree.entry(idx).or_default();
            e.0 = e.1 + v;
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        self.update(start, end - 1, 1, 0, 10_0000_0000, 1);
        if self.tree.get(&1).unwrap().0 > 2 {
            self.update(start, end - 1, -1, 0, 10_0000_0000, 1);
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut mct = MyCalendarTwo::new();
        assert_eq!(true, mct.book(10, 20));
        assert_eq!(true, mct.book(50, 60));
        assert_eq!(true, mct.book(10, 40));
        assert_eq!(false, mct.book(5, 15));
        assert_eq!(true, mct.book(5, 10));
        assert_eq!(true, mct.book(25, 55));
    }
}
