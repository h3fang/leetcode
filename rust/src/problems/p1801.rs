pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut buy: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut sell: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        for o in orders {
            let mut amount = o[1];
            if o[2] == 0 {
                while let Some(&(Reverse(p), n)) = sell.peek() {
                    if p > o[0] {
                        break;
                    }
                    match amount.cmp(&n) {
                        std::cmp::Ordering::Less => {
                            sell.pop();
                            sell.push((Reverse(p), n - amount));
                            amount = 0;
                            break;
                        }
                        std::cmp::Ordering::Equal => {
                            sell.pop();
                            amount = 0;
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            sell.pop();
                            amount -= n;
                        }
                    }
                }
                if amount > 0 {
                    buy.push((o[0], amount));
                }
            } else {
                while let Some(&(p, n)) = buy.peek() {
                    if p < o[0] {
                        break;
                    }
                    match amount.cmp(&n) {
                        std::cmp::Ordering::Less => {
                            buy.pop();
                            buy.push((p, n - amount));
                            amount = 0;
                            break;
                        }
                        std::cmp::Ordering::Equal => {
                            buy.pop();
                            amount = 0;
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            buy.pop();
                            amount -= n;
                        }
                    }
                }
                if amount > 0 {
                    sell.push((Reverse(o[0]), amount));
                }
            }
        }
        buy.iter()
            .map(|e| e.1)
            .chain(sell.iter().map(|e| e.1))
            .fold(0, |acc, n| (acc + n) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let orders = [[10, 5, 0], [15, 2, 1], [25, 1, 1], [30, 4, 0]]
            .iter()
            .map(|o| o.to_vec())
            .collect();
        assert_eq!(6, Solution::get_number_of_backlog_orders(orders));
    }

    #[test]
    fn case2() {
        let orders = [[7, 1000000000, 1], [15, 3, 0], [5, 999999995, 0], [5, 1, 1]]
            .iter()
            .map(|o| o.to_vec())
            .collect();
        assert_eq!(999999984, Solution::get_number_of_backlog_orders(orders));
    }
}
