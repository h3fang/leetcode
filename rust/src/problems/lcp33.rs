pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn store_water(mut bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        let mut q = BinaryHeap::new();
        let mut upgrade = 0;
        for (i, &v) in vat.iter().enumerate() {
            if v > 0 {
                if bucket[i] == 0 {
                    upgrade += 1;
                    bucket[i] += 1;
                }
                q.push(((v + bucket[i] - 1) / bucket[i], i));
            }
        }
        if q.is_empty() {
            return 0;
        }
        let mut result = i32::MAX;
        while upgrade < result {
            let (c, i) = q.pop().unwrap();
            result = result.min(c + upgrade);
            if c == 1 {
                break;
            }
            let b2 = (vat[i] + c - 2) / (c - 1);
            upgrade += b2 - bucket[i];
            bucket[i] = b2;
            q.push(((vat[i] + b2 - 1) / b2, i));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::store_water(vec![1, 3], vec![6, 8]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::store_water(vec![9, 0, 1], vec![0, 2, 2]));
    }
}
