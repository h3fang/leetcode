pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(mut piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut f = vec![0; k as usize + 1];
        let mut sum = 0;
        for p in piles.iter_mut() {
            for i in 1..p.len() {
                p[i] += p[i - 1];
            }
            sum = (sum + p.len()).min(k as usize);
            for j in (1..=sum).rev() {
                let max = (0..p.len().min(j))
                    .map(|k| p[k] + f[j - k - 1])
                    .max()
                    .unwrap_or(0);
                f[j] = f[j].max(max);
            }
        }
        f[k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let piles = vec![vec![1, 100, 3], vec![7, 8, 9]];
        let k = 2;
        assert_eq!(101, Solution::max_value_of_coins(piles, k));
    }

    #[test]
    fn case2() {
        let piles = vec![
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![1, 1, 1, 1, 1, 1, 700],
        ];
        let k = 7;
        assert_eq!(706, Solution::max_value_of_coins(piles, k));
    }
}
