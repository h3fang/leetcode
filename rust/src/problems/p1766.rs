pub struct Solution;

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut gcds = vec![vec![]; 51];
        for x in 1..51 {
            for y in 1..51 {
                if gcd(x, y) == 1 {
                    gcds[x as usize].push(y);
                }
            }
        }

        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }

        let mut result = vec![-1; n];
        let mut ancestors = vec![(0, 0); 51];

        #[allow(clippy::too_many_arguments)]
        fn dfs(
            nums: &[i32],
            g: &[Vec<i32>],
            x: i32,
            p: i32,
            d: i32,
            result: &mut [i32],
            ancestors: &mut [(i32, i32)],
            gcds: &[Vec<i32>],
        ) {
            let v = nums[x as usize];
            let mut max_depth = 0;
            for &j in &gcds[v as usize] {
                let (d1, y) = ancestors[j as usize];
                if d1 > max_depth {
                    max_depth = d1;
                    result[x as usize] = y;
                }
            }
            let tmp = ancestors[v as usize];
            ancestors[v as usize] = (d, x);
            for &y in &g[x as usize] {
                if y != p {
                    dfs(nums, g, y, x, d + 1, result, ancestors, gcds);
                }
            }
            ancestors[v as usize] = tmp;
        }

        dfs(&nums, &g, 0, -1, 1, &mut result, &mut ancestors, &gcds);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [1, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![-1, 0, 0, 1],
            Solution::get_coprimes(vec![2, 3, 3, 2], edges)
        );
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![-1, 0, -1, 0, 0, 0, -1],
            Solution::get_coprimes(vec![5, 6, 10, 2, 3, 6, 15], edges)
        );
    }
}
