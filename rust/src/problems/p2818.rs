pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn quick_pow(x: i32, mut n: i64) -> i64 {
    let mut x = x as i64;
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

fn f(x: usize) -> Vec<i32> {
    let mut ans = vec![0; x + 1];
    for i in 2..=x {
        if ans[i] == 0 {
            for j in (i..=x).step_by(i) {
                ans[j] += 1;
            }
        }
    }
    ans
}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let max = *nums.iter().max().unwrap();
        let scores = f(max as usize);

        let n = nums.len();
        let mut s = Vec::with_capacity(n);

        let mut left = vec![-1; n];
        let mut right = vec![n as i64; n];
        for (i, &x) in nums.iter().enumerate() {
            while !s.is_empty() && scores[nums[*s.last().unwrap()] as usize] < scores[x as usize] {
                let j = s.pop().unwrap();
                right[j] = i as i64;
            }
            if let Some(&j) = s.last() {
                left[i] = j as i64;
            }
            s.push(i);
        }

        let mut ids: Vec<usize> = (0..n).collect();
        ids.sort_unstable_by_key(|&i| nums[i]);
        let (mut ans, mut k) = (1, k as i64);
        for i in ids.into_iter().rev() {
            let r = (i as i64 - left[i]) * (right[i] - i as i64);
            let r = r.min(k);
            ans = (ans * quick_pow(nums[i], r)) % MOD;
            k -= r;
            if k == 0 {
                break;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(81, Solution::maximum_score(vec![8, 3, 9, 3, 8], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(
            4788,
            Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3)
        );
    }
}
