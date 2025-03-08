pub struct Solution;

impl Solution {
    pub fn maximum_beauty(
        mut flowers: Vec<i32>,
        mut new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        flowers.sort_unstable();

        let n = flowers.len() as i64;
        let full = full as i64;
        let partial = partial as i64;

        let mut f = flowers.partition_point(|&x| x < target);
        if f == 0 {
            return full * n;
        }

        let mut cost = vec![0; f];
        for i in 1..f {
            cost[i] = cost[i - 1] + (i as i64) * (flowers[i] - flowers[i - 1]) as i64;
        }
        if cost[f - 1] + (target - flowers[f - 1]) as i64 * f as i64 <= new_flowers {
            return (full * n).max(full * (n - 1) + (target as i64 - 1) * partial);
        }

        f -= 1;

        let mut result = 0;
        while new_flowers > 0 {
            let p = cost[..=f].partition_point(|&x| x <= new_flowers) - 1;
            let top = (flowers[p] as i64 + (new_flowers - cost[p]) / (p as i64 + 1))
                .min(target as i64 - 1);
            result = result.max(full * (n - f as i64 - 1) + partial * top);
            new_flowers -= (target - flowers[f]) as i64;
            if f == 0 {
                break;
            }
            f -= 1;
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

    #[test]
    fn case3() {
        assert_eq!(
            58,
            Solution::maximum_beauty(vec![5, 5, 15, 1, 9], 36, 12, 9, 2)
        );
    }
}
