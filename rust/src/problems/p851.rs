use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        fn dfs(map: &HashMap<i32, Vec<i32>>, p: i32, quiet: &[i32], result: &mut [i32]) {
            if result[p as usize] == -1 {
                if let Some(richer) = map.get(&p) {
                    let mut quietest = p;
                    let mut q = quiet[p as usize];
                    for &r in richer {
                        dfs(map, r, quiet, result);
                        let quiter = result[r as usize];
                        if quiet[quiter as usize] < q {
                            q = quiet[quiter as usize];
                            quietest = quiter;
                        }
                    }
                    result[p as usize] = quietest;
                } else {
                    result[p as usize] = p;
                }
            }
        }
        let n = quiet.len();
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for r in richer {
            map.entry(r[1]).or_default().push(r[0]);
        }
        let mut result = vec![-1; n];
        for p in 0..n as i32 {
            dfs(&map, p, &quiet, &mut result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let richer = [[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]];
        let richer = richer.iter().map(|r| r.to_vec()).collect();
        let quiet = vec![3, 2, 5, 4, 6, 1, 7, 0];
        let expected = vec![5, 5, 2, 5, 4, 5, 6, 7];
        assert_eq!(expected, Solution::loud_and_rich(richer, quiet));
    }

    #[test]
    fn case2() {
        let richer = [[0; 0]; 0];
        let richer = richer.iter().map(|r| r.to_vec()).collect();
        let quiet = vec![0];
        let expected = vec![0];
        assert_eq!(expected, Solution::loud_and_rich(richer, quiet));
    }
}
