pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn grid_illumination(_n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut h, mut v, mut l, mut r) = (
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        );
        let mut lamps = lamps
            .into_iter()
            .map(|e| (e[0], e[1]))
            .collect::<HashSet<_>>();

        lamps.iter().for_each(|&(x, y)| {
            *v.entry(x).or_insert(0) += 1;
            *h.entry(y).or_insert(0) += 1;
            *l.entry(x + y).or_insert(0) += 1;
            *r.entry(x - y).or_insert(0) += 1;
        });

        let del = |mp: &mut HashMap<i32, i32>, key: i32| {
            if let Some(t) = mp.get_mut(&key) {
                *t -= 1;
                if t.eq(&&0) {
                    mp.remove(&key);
                }
            }
        };

        queries
            .into_iter()
            .map(|e| {
                let (x, y) = (e[0], e[1]);
                let res = (v.contains_key(&x)
                    || h.contains_key(&y)
                    || l.contains_key(&(x + y))
                    || r.contains_key(&(x - y))) as i32;
                for nx in [x - 1, x, x + 1] {
                    for ny in [y - 1, y, y + 1] {
                        if lamps.contains(&(nx, ny)) {
                            del(&mut v, nx);
                            del(&mut h, ny);
                            del(&mut l, nx + ny);
                            del(&mut r, nx - ny);
                            lamps.remove(&(nx, ny));
                        }
                    }
                }
                res
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 5;
        let lamps = [[0, 0], [4, 4]];
        let queries = [[1, 1], [1, 0]];
        let lamps = lamps.iter().map(|l| l.to_vec()).collect();
        let queries = queries.iter().map(|l| l.to_vec()).collect();
        assert_eq!(vec![1, 0], Solution::grid_illumination(n, lamps, queries));
    }

    #[test]
    fn case2() {
        let n = 5;
        let lamps = [[0, 0], [4, 4]];
        let queries = [[1, 1], [1, 1]];
        let lamps = lamps.iter().map(|l| l.to_vec()).collect();
        let queries = queries.iter().map(|l| l.to_vec()).collect();
        assert_eq!(vec![1, 1], Solution::grid_illumination(n, lamps, queries));
    }

    #[test]
    fn case3() {
        let n = 5;
        let lamps = [[0, 0], [0, 4]];
        let queries = [[0, 4], [0, 1], [1, 4]];
        let lamps = lamps.iter().map(|l| l.to_vec()).collect();
        let queries = queries.iter().map(|l| l.to_vec()).collect();
        assert_eq!(
            vec![1, 1, 0],
            Solution::grid_illumination(n, lamps, queries)
        );
    }
}
