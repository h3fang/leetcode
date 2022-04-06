pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut nums = vec![0i64; 101];
        for n in arr {
            nums[n as usize] += 1;
        }
        let mut result = 0i64;
        for i in 0..=100 {
            let x = nums[i as usize];
            if x == 0 {
                continue;
            }
            let t = target - i;
            let mut j = i;
            let mut k = 100;
            while j <= k {
                if nums[j as usize] == 0 {
                    j += 1;
                    continue;
                } else if nums[k as usize] == 0 {
                    k -= 1;
                    continue;
                }
                let y = nums[j as usize];
                let z = nums[k as usize];
                match (j + k).cmp(&t) {
                    std::cmp::Ordering::Less => j += 1,
                    std::cmp::Ordering::Equal => {
                        if i == j && j == k {
                            result = (result + x * (x - 1) * (x - 2) / 6) % MOD;
                        } else if i == j {
                            result = (result + x * (x - 1) / 2 * z) % MOD;
                        } else if j == k {
                            result = (result + y * (y - 1) / 2 * x) % MOD;
                        } else if i == k {
                            result = (result + x * (x - 1) / 2 * y) % MOD;
                        } else {
                            result = (result + x * y * z) % MOD;
                        }
                        j += 1;
                        k -= 1;
                    }
                    std::cmp::Ordering::Greater => k -= 1,
                }
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            20,
            Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5));
    }
}
