pub struct Solution;

fn dfs(g: &[Vec<i32>], present: &[i32], future: &[i32], budget: usize, x: i32) -> [Vec<i32>; 2] {
    let mut f1 = [
        vec![i32::MIN / 2; budget + 1],
        vec![i32::MIN / 2; budget + 1],
    ];
    f1[0][0] = 0;
    f1[1][0] = 0;
    for &y in &g[x as usize] {
        let fy = dfs(g, present, future, budget, y);
        for (k, fy) in fy.into_iter().enumerate() {
            let mut f = vec![i32::MIN / 2; budget + 1];
            f[0] = 0;
            for (bud, &profit) in fy.iter().enumerate() {
                if profit < 0 {
                    continue;
                }
                for b1 in bud..budget + 1 {
                    f[b1] = f[b1].max(f1[k][b1 - bud] + profit);
                }
            }
            f1[k] = f;
        }
    }
    let mut f = [f1[0].clone(), f1[0].clone()];
    for (k, r) in f.iter_mut().enumerate() {
        let cost = present[x as usize] / (k as i32 + 1);
        for j in cost as usize..budget + 1 {
            r[j] = r[j].max(f1[1][j - cost as usize] + future[x as usize] - cost);
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
        *f[0].iter().max().unwrap()
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
