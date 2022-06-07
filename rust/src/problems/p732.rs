use std::collections::BTreeMap;

#[derive(Default)]
pub struct MyCalendarThree {
    tps: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn book(&mut self, start: i32, end: i32) -> i32 {
        self.tps
            .insert(start, self.tps.get(&start).unwrap_or(&0) + 1);
        self.tps.insert(end, self.tps.get(&end).unwrap_or(&0) - 1);
        self.tps
            .iter()
            .fold((0, 0), |(max, curr), (_, v)| (max.max(curr + v), curr + v))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut mc = MyCalendarThree::new();
        assert_eq!(1, mc.book(10, 20));
        assert_eq!(1, mc.book(50, 60));
        assert_eq!(2, mc.book(10, 40));
        assert_eq!(3, mc.book(5, 15));
        assert_eq!(3, mc.book(5, 10));
        assert_eq!(3, mc.book(25, 55));
    }
}
