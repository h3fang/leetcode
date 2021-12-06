use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(g: &mut HashMap<i32, Vec<i32>>, curr: i32, result: &mut Vec<Vec<i32>>) {
            if let Some(children) = g.get_mut(&curr) {
                let children = children as *mut Vec<i32>;
                while let Some(n) = unsafe { (*children).pop() } {
                    dfs(g, n, result);
                    result.push(vec![curr, n]);
                }
            }
        }
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut degree: HashMap<i32, (usize, usize)> = HashMap::new();
        for p in &pairs {
            g.entry(p[0]).or_default().push(p[1]);
            degree.entry(p[0]).or_default().1 += 1;
            degree.entry(p[1]).or_default().0 += 1;
        }
        let start = degree
            .iter()
            .find(|(_, v)| v.1 == v.0 + 1)
            .map(|e| e.0)
            .cloned()
            .unwrap_or(pairs[0][0]);
        let mut result = vec![];
        dfs(&mut g, start, &mut result);
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify(pairs: &[Vec<i32>], result: &[Vec<i32>]) {
        for w in result.windows(2) {
            assert!(pairs.contains(&w[0]));
            assert!(pairs.contains(&w[1]));
            assert_eq!(w[0][1], w[1][0]);
        }
    }

    #[test]
    fn case1() {
        let pairs = [[5, 1], [4, 5], [11, 9], [9, 4]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        verify(&pairs, &Solution::valid_arrangement(pairs.clone()));
    }

    #[test]
    fn case2() {
        let pairs = [[1, 3], [3, 2], [2, 1]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        verify(&pairs, &Solution::valid_arrangement(pairs.clone()));
    }

    #[test]
    fn case3() {
        let pairs = [[1, 2], [1, 3], [2, 1]];
        let pairs = pairs.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        verify(&pairs, &Solution::valid_arrangement(pairs.clone()));
    }
}
