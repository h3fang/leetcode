pub struct Solution;

fn dfs(g: &[Vec<i32>], present: &[i32], future: &[i32], budget: usize, x: i32) -> Vec<[i32; 2]> {
    let mut f1 = vec![[0; 2]; budget + 1];
    for &y in &g[x as usize] {
        let fy = dfs(g, present, future, budget, y);
        for j in (0..=budget).rev() {
            for b1 in 0..=j {
                for k in 0..2 {
                    f1[j][k] = f1[j][k].max(f1[j - b1][k] + fy[b1][k]);
                }
            }
        }
    }
    let mut f = vec![[0; 2]; budget + 1];
    for j in 0..=budget {
        for k in 0..2 {
            let cost = present[x as usize] / (k as i32 + 1);
            if j as i32 >= cost {
                f[j][k] = f1[j][0].max(f1[j - cost as usize][1] + future[x as usize] - cost);
            } else {
                f[j][k] = f1[j][0];
            }
        }
    }
    f
}

impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let mut g = vec![vec![]; n as usize + 1];
        for h in hierarchy {
            g[h[0] as usize - 1].push(h[1] - 1);
        }
        let budget = budget as usize;
        let f = dfs(&g, &present, &future, budget, 0);
        f[budget][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            5,
            Solution::max_profit(2, vec![1, 2], vec![4, 3], vec![vec![1, 2]], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::max_profit(2, vec![3, 4], vec![5, 8], vec![vec![1, 2]], 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            10,
            Solution::max_profit(
                2,
                vec![4, 6, 8],
                vec![7, 9, 11],
                vec![vec![1, 2], vec![1, 3]],
                10
            )
        );
    }
}
