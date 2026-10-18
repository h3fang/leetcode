pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn qpow(mut x: i64, mut n: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n % 2 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prev = vec![0; n + 2];
        let mut next = vec![0; n + 2];
        let mut size = vec![0; n + 2];

        for (i, (p, n)) in prev.iter_mut().zip(next.iter_mut()).enumerate() {
            *p = i.saturating_sub(1);
            *n = i + 1;
        }

        let mut prod = 1;

        for x in nums.into_iter().rev() {
            let x = x as usize;
            size[x] = 1;

            let l = prev[x];
            if x > 0 && size[l] > 0 {
                size[x] += size[l];

                let p = prev[l];
                prev[x] = p;
                next[p] = x;
            }

            let r = next[x];
            if size[r] > 0 {
                size[x] += size[r];

                let n = next[r];
                prev[n] = x;
                next[x] = n;
            }

            prod = (prod * size[x]) % MOD;
        }

        let mut fac = 1;
        for i in 2..n + 1 {
            fac = (fac * i as i64) % MOD;
        }

        let inv = qpow(prod, MOD - 2);
        let ans = (fac * inv - 1 + MOD) % MOD;
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::num_of_ways(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::num_of_ways(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::num_of_ways(vec![1, 2, 3]));
    }
}
