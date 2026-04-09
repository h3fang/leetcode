pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn pow(mut x: i64, mut n: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}

impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let b = queries.len().isqrt();
        let mut groups = vec![vec![]; b];
        for q in queries {
            let k = q[2] as usize;
            if k < b {
                groups[k].push((q[0], q[1], q[3]));
            } else {
                for i in (q[0] as usize..=q[1] as usize).step_by(k) {
                    nums[i] = ((nums[i] as i64 * q[3] as i64) % MOD) as i32;
                }
            }
        }

        let mut diff = vec![0; n + 1];

        for (k, g) in groups.iter().enumerate().skip(1) {
            if g.is_empty() {
                continue;
            }

            let mut buckets = vec![vec![]; k];
            for t in g {
                buckets[(t.0 as usize) % k].push(*t);
            }

            for (start, bucket) in buckets.iter().enumerate() {
                if bucket.is_empty() {
                    continue;
                }

                if bucket.len() == 1 {
                    let (l, r, v) = bucket[0];
                    for i in (l as usize..=r as usize).step_by(k) {
                        nums[i] = ((nums[i] as i64 * v as i64) % MOD) as i32;
                    }
                    continue;
                }

                let m = (n - start - 1) / k + 1;
                diff[..m].fill(1);

                for &(l, r, v) in bucket {
                    diff[l as usize / k] = (diff[l as usize / k] * v as i64) % MOD;
                    let r = (r as usize - start) / k + 1;
                    diff[r] = (diff[r] * pow(v as i64, MOD - 2)) % MOD;
                }

                let mut mul = 1;
                for (i, &d) in diff.iter().enumerate().take(m) {
                    mul = (mul * d) % MOD;
                    let j = start + i * k;
                    nums[j] = ((nums[j] as i64 * mul) % MOD) as i32;
                }
            }
        }

        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 1, 1];
        let queries = [[0, 2, 1, 4]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(4, Solution::xor_after_queries(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![2, 3, 1, 5, 4];
        let queries = [[1, 4, 2, 3], [0, 2, 1, 2]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(31, Solution::xor_after_queries(nums, queries));
    }
}
