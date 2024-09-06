pub struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let n = group_sizes.len();
        let mut gs = group_sizes
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<Vec<_>>();
        gs.sort_unstable();
        let mut result = vec![];
        let mut i = 0;
        while i < n {
            let size = gs[i].0 as usize;
            let mut g = Vec::with_capacity(size);
            for _ in 0..size {
                g.push(gs[i].1 as i32);
                i += 1;
            }
            result.push(g);
        }

        result
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    fn assert_valid(group_sizes: &[i32], groups: Vec<Vec<i32>>) {
        let mut m = HashMap::new();
        println!("{groups:?}");
        for g in &groups {
            for i in g {
                assert!(!m.contains_key(i));
                m.insert(*i, g.len() as i32);
            }
        }
        for (i, &size) in group_sizes.iter().enumerate() {
            assert_eq!(size, *m.get(&(i as i32)).unwrap());
        }
    }

    #[test]
    fn case1() {
        let group_sizes = vec![3, 3, 3, 3, 3, 1, 3];
        assert_valid(
            &group_sizes,
            Solution::group_the_people(group_sizes.clone()),
        );
    }
}
