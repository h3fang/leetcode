use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; positions.len()];
        let mut map = BTreeMap::new();
        map.insert(0, 0);
        for (i, p) in positions.iter().enumerate() {
            let (left, side) = (p[0], p[1]);
            let right = left + side - 1;
            let mut inside = vec![];
            let mut max = side;
            let l = *map.range(..=left).last().unwrap().0;
            let last_h = *map.range(..=right).last().unwrap().1;
            for (&p, &h) in map.range(l..=right) {
                max = max.max(h + side);
                inside.push(p);
            }
            inside.into_iter().for_each(|i| {
                if i >= left {
                    map.remove(&i);
                }
            });
            map.insert(left, max);
            if let Some((&p, _)) = map.range(right + 1..).next() {
                if p > right + 1 {
                    map.insert(right + 1, last_h);
                }
            } else {
                map.insert(right + 1, last_h);
            }
            result[i] = max;
            if i > 0 {
                result[i] = result[i].max(result[i - 1]);
            }
            println!("{:?}", map);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let positions = vec![vec![1, 2], vec![2, 3], vec![6, 1]];
        assert_eq!(vec![2, 5, 5], Solution::falling_squares(positions));
    }
}
