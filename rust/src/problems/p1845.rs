use std::{cmp::Reverse, collections::BinaryHeap};

pub struct SeatManager {
    min: i32,
    q: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    pub fn new(_n: i32) -> Self {
        Self {
            min: 1,
            q: Default::default(),
        }
    }

    pub fn reserve(&mut self) -> i32 {
        if let Some(Reverse(s)) = self.q.pop() {
            s
        } else {
            let s = self.min;
            self.min += 1;
            s
        }
    }

    pub fn unreserve(&mut self, seat_number: i32) {
        self.q.push(Reverse(seat_number));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut sm = SeatManager::new(5);
        assert_eq!(1, sm.reserve());
        assert_eq!(2, sm.reserve());
        sm.unreserve(2);
        assert_eq!(2, sm.reserve());
        assert_eq!(3, sm.reserve());
        assert_eq!(4, sm.reserve());
        assert_eq!(5, sm.reserve());
        sm.unreserve(5);
    }
}
