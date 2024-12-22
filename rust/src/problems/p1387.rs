pub struct Solution;

use std::collections::HashMap;

fn weight(x: i32, m: &mut HashMap<i32, i32>) -> i32 {
    if x == 1 {
        return 0;
    }
    if let Some(&r) = m.get(&x) {
        return r;
    }

    let result = if x % 2 == 0 {
        1 + weight(x / 2, m)
    } else {
        1 + weight(x * 3 + 1, m)
    };
    m.insert(x, result);
    result
}

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut m = HashMap::with_capacity(1000);
        let mut nums = (lo..=hi)
            .map(|x| (weight(x, &mut m), x))
            .collect::<Vec<_>>();
        nums.sort_unstable();
        nums[k as usize - 1].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::get_kth(12, 15, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::get_kth(7, 11, 4));
    }
}
