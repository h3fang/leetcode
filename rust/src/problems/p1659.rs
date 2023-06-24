pub struct Solution;

const SCORE: [[i32; 3]; 3] = [[0, 0, 0], [0, -60, -10], [0, -10, 40]];

impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            m: i32,
            n: i32,
            d: &mut [[[[i32; 7]; 7]; 243]; 30],
            p: &[i32],
            pos: i32,
            mask: i32,
            iv: i32,
            ev: i32,
        ) -> i32 {
            if pos == n * m || (iv == 0 && ev == 0) {
                return 0;
            }
            let r = d[pos as usize][mask as usize][iv as usize][ev as usize];
            if r != -1 {
                return r;
            }

            let mut result = 0;
            let up = mask / p[n as usize - 1];
            let mut left = mask % 3;
            if pos % n == 0 {
                left = 0;
            }
            for i in 0..3 {
                if (i == 1 && iv == 0) || (i == 2 && ev == 0) {
                    continue;
                }
                let next_mask = mask % p[n as usize - 1] * 3 + i;
                let mut score_num = dfs(
                    m,
                    n,
                    d,
                    p,
                    pos + 1,
                    next_mask,
                    iv - i32::from(i == 1),
                    ev - i32::from(i == 2),
                ) + SCORE[up as usize][i as usize]
                    + SCORE[left as usize][i as usize];
                if i == 1 {
                    score_num += 120;
                } else if i == 2 {
                    score_num += 40;
                }
                result = result.max(score_num);
            }
            d[pos as usize][mask as usize][iv as usize][ev as usize] = result;
            result
        }
        let mut p = [0; 5];
        let mut d = [[[[-1; 7]; 7]; 243]; 30];
        p[0] = 1;
        for i in 1..n as usize {
            p[i] = p[i - 1] * 3;
        }
        dfs(m, n, &mut d, &p, 0, 0, introverts_count, extroverts_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(240, Solution::get_max_grid_happiness(2, 3, 1, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(260, Solution::get_max_grid_happiness(3, 1, 2, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(240, Solution::get_max_grid_happiness(2, 2, 4, 0));
    }
}
