pub struct StockSpanner {
    curr: i32,
    s: Vec<(i32, i32)>,
}

impl StockSpanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            curr: 0,
            s: vec![(i32::MAX, -1)],
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        while self.s.last().unwrap().0 <= price {
            self.s.pop();
        }
        let r = self.curr - self.s.last().unwrap().1;
        self.s.push((price, self.curr));
        self.curr += 1;
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ss = StockSpanner::new();
        assert_eq!(1, ss.next(100));
        assert_eq!(1, ss.next(80));
        assert_eq!(1, ss.next(60));
        assert_eq!(2, ss.next(70));
        assert_eq!(1, ss.next(60));
        assert_eq!(4, ss.next(75));
        assert_eq!(6, ss.next(85));
    }
}
