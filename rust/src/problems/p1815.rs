pub struct Solution;

use std::collections::HashMap;

const KWIDTH: i64 = 5;
const KWIDTH_MASK: i64 = (1 << KWIDTH) - 1;

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        fn dfs(tab: &mut HashMap<i64, i32>, batch_size: i64, mask: i64) -> i32 {
            if let Some(x) = tab.get(&mask) {
                return *x;
            }
            let total = (1..batch_size).fold(0, |total, i| {
                let amount = mask.overflowing_shr(((i - 1) * KWIDTH) as u32).0 & KWIDTH_MASK;
                total + i * amount
            });
            let best = (1..batch_size).fold(0, |best, i| {
                let amount = mask.overflowing_shr(((i - 1) * KWIDTH) as u32).0 & KWIDTH_MASK;
                if amount > 0 {
                    let mut result = dfs(
                        tab,
                        batch_size,
                        mask - 1i64.overflowing_shl(((i - 1) * KWIDTH) as u32).0,
                    );
                    if (total - i) % batch_size == 0 {
                        result += 1;
                    }
                    best.max(result)
                } else {
                    best
                }
            });
            tab.entry(mask).or_insert(best);
            best
        }
        let mut tab = HashMap::with_capacity(1 << 16);
        let mut cnt = vec![0; batch_size as usize];
        groups
            .into_iter()
            .for_each(|i| cnt[(i % batch_size) as usize] += 1);
        let start = (1..batch_size)
            .rev()
            .fold(0, |start, i| (start << KWIDTH) | cnt[i as usize]);
        tab.entry(0).or_insert(0);
        dfs(&mut tab, batch_size as i64, start) + cnt[0] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_happy_groups(3, vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::max_happy_groups(4, vec![1, 3, 2, 5, 2, 2, 1, 6])
        );
    }
}
