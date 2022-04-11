pub struct Solution;

impl Solution {
    pub fn maximum_beauty(
        mut flowers: Vec<i32>,
        mut new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        fn bisec_right(arr: &[i64], target: i64) -> usize {
            let mut left = 0;
            let mut right = arr.len() - 1;
            let mut result = arr.len();
            while left <= right {
                let mid = (left + right) / 2;
                match arr[mid].cmp(&target) {
                    std::cmp::Ordering::Greater => {
                        result = mid;
                        right = mid - 1;
                    }
                    _ => left = mid + 1,
                }
            }
            result
        }

        flowers.iter_mut().for_each(|f| *f = (*f).min(target));
        flowers.sort_unstable();

        let n = flowers.len() as i64;
        let full = full as i64;
        let partial = partial as i64;

        let mut j = flowers.len() - 1;
        while flowers[j] == target {
            if j == 0 {
                return full as i64 * n;
            }
            j -= 1;
        }

        let sum = flowers.iter().fold(0i64, |acc, &n| acc + n as i64);
        if new_flowers >= target as i64 * n - sum {
            return (full * n).max(full * (n - 1) + partial * (target as i64 - 1));
        }

        let mut cost = vec![0; flowers.len()];
        for i in 1..flowers.len() {
            cost[i] = cost[i - 1] + (flowers[i] - flowers[i - 1]) as i64 * i as i64;
        }
        let mut result = 0;
        while new_flowers > 0 {
            let idx = j.min(bisec_right(&cost, new_flowers) - 1);
            let top = (flowers[idx] as i64 + (new_flowers - cost[idx]) / (idx as i64 + 1))
                .min(target as i64 - 1);
            result =
                result.max(full as i64 * (flowers.len() - j - 1) as i64 + partial as i64 * top);
            new_flowers -= (target - flowers[j]) as i64;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(14, Solution::maximum_beauty(vec![1, 3, 1, 1], 7, 6, 12, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(30, Solution::maximum_beauty(vec![2, 4, 5, 3], 10, 5, 2, 6));
    }
}
