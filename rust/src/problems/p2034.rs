use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

#[derive(Eq, Clone)]
pub struct Price {
    timestamp: i32,
    price: i32,
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(Default)]
pub struct StockPrice {
    max_heap: BinaryHeap<Price>,
    min_heap: BinaryHeap<Reverse<Price>>,
    prices: HashMap<i32, i32>,
    last_timestamp: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    pub fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
            prices: HashMap::new(),
            last_timestamp: 0,
        }
    }

    pub fn update(&mut self, timestamp: i32, price: i32) {
        self.last_timestamp = self.last_timestamp.max(timestamp);
        self.prices.insert(timestamp, price);
        let p = Price { timestamp, price };
        self.max_heap.push(p.clone());
        self.min_heap.push(Reverse(p));
    }

    pub fn current(&self) -> i32 {
        *self.prices.get(&self.last_timestamp).unwrap()
    }

    pub fn maximum(&mut self) -> i32 {
        let mut p = self.max_heap.peek().unwrap();
        while self.prices.get(&p.timestamp).unwrap() != &p.price {
            self.max_heap.pop();
            p = self.max_heap.peek().unwrap();
        }
        p.price
    }

    pub fn minimum(&mut self) -> i32 {
        let mut p = self.min_heap.peek().unwrap();
        while self.prices.get(&p.0.timestamp).unwrap() != &p.0.price {
            self.min_heap.pop();
            p = self.min_heap.peek().unwrap();
        }
        p.0.price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut stockPrice = StockPrice::new();
        stockPrice.update(1, 10); // Timestamps are [1] with corresponding prices [10].
        stockPrice.update(2, 5); // Timestamps are [1,2] with corresponding prices [10,5].
        assert_eq!(5, stockPrice.current()); // return 5, the latest timestamp is 2 with the price being 5.
        assert_eq!(10, stockPrice.maximum()); // return 10, the maximum price is 10 at timestamp 1.
        stockPrice.update(1, 3); // The previous timestamp 1 had the wrong price, so it is updated to 3.
                                 // Timestamps are [1,2] with corresponding prices [3,5].
        assert_eq!(5, stockPrice.maximum()); // return 5, the maximum price is 5 after the correction.
        stockPrice.update(4, 2); // Timestamps are [1,2,4] with corresponding prices [3,5,2].
        assert_eq!(2, stockPrice.minimum()); // return 2, the minimum price is 2 at timestamp 4.
    }
}
