pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        fn dfs(
            price: &[i32],
            special: &[Vec<i32>],
            needs: Vec<i32>,
            cache: &mut HashMap<Vec<i32>, i32>,
        ) -> i32 {
            if needs.iter().all(|&e| e == 0) {
                return 0;
            }
            if let Some(&e) = cache.get(&needs) {
                return e;
            }
            let mut r: i32 = needs.iter().enumerate().map(|(i, &x)| x * price[i]).sum();
            for s in special {
                if s.iter().zip(&needs).any(|(a, b)| a > b) {
                    continue;
                }
                let next = s.iter().zip(&needs).map(|(a, b)| b - a).collect();
                let r1 = s.last().unwrap() + dfs(price, special, next, cache);
                r = r.min(r1);
            }
            cache.insert(needs, r);
            r
        }
        dfs(&price, &special, needs, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let price = vec![2, 5];
        let special = [[3, 0, 5], [1, 2, 10]].iter().map(|s| s.to_vec()).collect();
        let needs = vec![3, 2];
        assert_eq!(14, Solution::shopping_offers(price, special, needs));
    }

    #[test]
    fn case2() {
        let price = vec![2, 3, 4];
        let special = [[1, 1, 0, 4], [2, 2, 1, 9]]
            .iter()
            .map(|s| s.to_vec())
            .collect();
        let needs = vec![1, 2, 1];
        assert_eq!(11, Solution::shopping_offers(price, special, needs));
    }
}
