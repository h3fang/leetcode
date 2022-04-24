pub struct Solution;

impl Solution {
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn cross(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
            (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0])
        }

        let n = trees.len();
        if n < 4 {
            return trees;
        }

        trees.sort_unstable();
        let mut result = vec![0];
        let mut used = vec![false; n];

        for i in 1..n {
            let mut m = result.len();
            while m > 1 && cross(&trees[result[m - 2]], &trees[result[m - 1]], &trees[i]) < 0 {
                used[result.pop().unwrap()] = false;
                m -= 1;
            }
            used[i] = true;
            result.push(i);
        }

        let bottom = result.len();

        for i in (0..n - 1).rev() {
            if !used[i] {
                let mut m = result.len();
                while m > bottom
                    && cross(&trees[result[m - 2]], &trees[result[m - 1]], &trees[i]) < 0
                {
                    used[result.pop().unwrap()] = false;
                    m -= 1;
                }
                used[i] = true;
                result.push(i);
            }
        }
        result.pop();
        result.iter().map(|i| trees[*i].clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let trees = [[1, 1], [2, 2], [2, 0], [2, 4], [3, 3], [4, 2]];
        let trees = trees.iter().map(|p| p.to_vec()).collect();
        let expected = [[1, 1], [2, 0], [4, 2], [3, 3], [2, 4]];
        let mut expected = expected.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::outer_trees(trees);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
