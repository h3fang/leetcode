use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn full_bloom_flowers(mut flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32> {
        flowers.sort_unstable();
        let mut i = 0;
        let mut q: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut p = persons
            .iter()
            .enumerate()
            .map(|(i, p)| (*p, i))
            .collect::<Vec<_>>();
        p.sort_unstable();
        let mut r = vec![0; persons.len()];
        p.into_iter().for_each(|(p, j)| {
            while i < flowers.len() && flowers[i][0] <= p {
                q.push(Reverse(flowers[i][1]));
                i += 1;
            }
            while !q.is_empty() && q.peek().unwrap().0 < p {
                q.pop();
            }
            r[j] = q.len() as i32;
        });
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let flowers = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let persons = vec![2, 3, 7, 11];
        assert_eq!(
            vec![1, 2, 2, 2],
            Solution::full_bloom_flowers(flowers, persons)
        );
    }

    #[test]
    fn case2() {
        let flowers = vec![vec![1, 10], vec![3, 3]];
        let persons = vec![3, 3, 2];
        assert_eq!(
            vec![2, 2, 1],
            Solution::full_bloom_flowers(flowers, persons)
        );
    }
}
