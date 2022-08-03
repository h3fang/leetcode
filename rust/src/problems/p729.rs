use std::collections::BTreeMap;

#[derive(Default)]
pub struct MyCalendar {
    cal: BTreeMap<i32, i32>,
}

impl MyCalendar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, &r2)) = self.cal.range(..end).last() {
            if r2 > start {
                return false;
            }
        }
        self.cal.insert(start, end);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut c = MyCalendar::new();
        assert_eq!(true, c.book(10, 20));
        assert_eq!(false, c.book(15, 25));
        assert_eq!(true, c.book(20, 30));
    }
}
