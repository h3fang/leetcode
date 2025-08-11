pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = Vec::with_capacity(31);
        while n > 0 {
            let lb = n & (-n);
            powers.push(lb as i64);
            n ^= lb;
        }
        let n = powers.len();
        let mut f = vec![vec![0; n]; n];
        for (i, f) in f.iter_mut().enumerate() {
            let mut p = 1;
            for (j, &y) in powers.iter().enumerate().skip(i) {
                p = (p * y) % MOD;
                f[j] = p;
            }
        }
        queries
            .into_iter()
            .map(|q| f[q[0] as usize][q[1] as usize] as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[0, 1], [2, 2], [0, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![2, 4, 64], Solution::product_queries(15, queries));
    }

    #[test]
    fn case2() {
        let queries = [[0, 0]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![2], Solution::product_queries(2, queries));
    }
}
