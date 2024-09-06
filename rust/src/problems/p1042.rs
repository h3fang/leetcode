pub struct Solution;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for p in paths {
            g[p[0] as usize - 1].push(p[1] - 1);
            g[p[1] as usize - 1].push(p[0] - 1);
        }
        let mut result = vec![0; n];
        for i in 0..n {
            let mut colored = [false; 5];
            for &next in &g[i] {
                colored[result[next as usize] as usize] = true;
            }
            for (j, &c) in colored.iter().enumerate().skip(1) {
                if !c {
                    result[i] = j as i32;
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: i32, paths: Vec<Vec<i32>>, colors: Vec<i32>) -> bool {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for p in paths {
            g[p[0] as usize - 1].push(p[1] - 1);
            g[p[1] as usize - 1].push(p[0] - 1);
        }
        for i in 0..n {
            let c = colors[i];
            for &next in &g[i] {
                if colors[next as usize] == c {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn case1() {
        let n = 3;
        let paths = [[1, 2], [2, 3], [3, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        let colors = Solution::garden_no_adj(n, paths.clone());
        assert!(check(n, paths, colors));
    }

    #[test]
    fn case2() {
        let n = 4;
        let paths = [[1, 2], [3, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        let colors = Solution::garden_no_adj(n, paths.clone());
        assert!(check(n, paths, colors));
    }

    #[test]
    fn case3() {
        let n = 4;
        let paths = [[1, 2], [2, 3], [3, 4], [4, 1], [1, 3], [2, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        let colors = Solution::garden_no_adj(n, paths.clone());
        assert!(check(n, paths, colors));
    }
}
