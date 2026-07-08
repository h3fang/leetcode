pub struct Solution;

const MOD: i64 = 10_0000_0007;

static POW10: [i64; 10_0001] = init_pow10();

const fn init_pow10() -> [i64; 10_0001] {
    let mut p = [1; 10_0001];
    let mut i = 1;
    while i < 10_0001 {
        p[i] = (p[i - 1] * 10) % MOD;
        i += 1;
    }
    p
}

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let mut sum = vec![0; n + 1];
        let mut pre = vec![0; n + 1];
        let mut nonzero = vec![0; n + 1];

        for (i, &b) in s.as_bytes().iter().enumerate() {
            let d = (b - b'0') as i64;
            sum[i + 1] = sum[i] + d;
            pre[i + 1] = if d > 0 {
                (pre[i] * 10 + d) % MOD
            } else {
                pre[i]
            };
            nonzero[i + 1] = nonzero[i] + u32::from(d > 0);
        }

        queries
            .iter()
            .map(|q| {
                let (l, r) = (q[0] as usize, q[1] as usize);
                let sum = sum[r + 1] - sum[l];
                let len = nonzero[r + 1] - nonzero[l];
                let x = (pre[r + 1] - (pre[l] * POW10[len as usize]) % MOD + MOD) % MOD;
                ((sum * x) % MOD) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "10203004".to_string();
        let queries = [[0, 7], [1, 3], [4, 6]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(vec![12340, 4, 9], Solution::sum_and_multiply(s, queries));
    }

    #[test]
    fn case2() {
        let s = "1000".to_string();
        let queries = [[0, 3], [1, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![1, 0], Solution::sum_and_multiply(s, queries));
    }

    #[test]
    fn case3() {
        let s = "9876543210".to_string();
        let queries = [[0, 9]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(vec![444444137], Solution::sum_and_multiply(s, queries));
    }
}
