pub struct Solution;

impl Solution {
    pub fn maximum_beauty(
        mut flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        flowers.sort_unstable();

        let n = flowers.len() as i64;
        let full = full as i64;
        let partial = partial as i64;

        let f = flowers.partition_point(|&x| x < target);
        if f == 0 {
            return full * n;
        }

        let cost: i64 = flowers[..f].iter().map(|&x| (target - x) as i64).sum();
        if cost <= new_flowers {
            return (full * n).max(full * (n - 1) + (target as i64 - 1) * partial);
        }
        let mut rem = new_flowers - cost;

        let (mut pre, mut i, mut result) = (0, 0, 0);
        for (j, x) in flowers[..f].iter().enumerate() {
            rem += (target - x).max(0) as i64;
            if rem < 0 {
                continue;
            }
            while i <= j && pre + rem >= flowers[i] as i64 * i as i64 {
                pre += flowers[i] as i64;
                i += 1;
            }
            let top = ((pre + rem) / (i as i64)).min(target as i64 - 1);
            result = result.max(full * (n - j as i64 - 1) + partial * top);
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
