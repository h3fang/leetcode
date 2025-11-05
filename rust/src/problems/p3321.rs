pub struct Solution;

use std::collections::{BTreeSet, HashMap};

fn add(
    cnt: &mut HashMap<i32, i32>,
    left: &mut BTreeSet<(i32, i32)>,
    right: &mut BTreeSet<(i32, i32)>,
    y: i32,
    sum: &mut i64,
) {
    let c = *cnt.get(&y).unwrap();
    if c == 0 {
        return;
    }
    if left.iter().next().is_some_and(|e| *e < (c, y)) {
        *sum += c as i64 * y as i64;
        left.insert((c, y));
    } else {
        right.insert((c, y));
    }
}

fn del(
    cnt: &mut HashMap<i32, i32>,
    left: &mut BTreeSet<(i32, i32)>,
    right: &mut BTreeSet<(i32, i32)>,
    y: i32,
    sum: &mut i64,
) {
    let c = cnt.get(&y).cloned().unwrap_or(0);
    if c == 0 {
        return;
    }
    if left.contains(&(c, y)) {
        *sum -= c as i64 * y as i64;
        left.remove(&(c, y));
    } else {
        right.remove(&(c, y));
    }
}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let (k, x) = (k as usize, x as usize);

        let mut sum = 0;
        let mut left: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut right: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut cnt: HashMap<i32, i32> = HashMap::with_capacity(2 * x);

        let mut ans = Vec::with_capacity(nums.len() + 1 - k);
        for (r, &y) in nums.iter().enumerate() {
            del(&mut cnt, &mut left, &mut right, y, &mut sum);
            *cnt.entry(y).or_default() += 1;
            add(&mut cnt, &mut left, &mut right, y, &mut sum);

            if r + 1 < k {
                continue;
            }
            let l = r + 1 - k;

            while !right.is_empty() && left.len() < x {
                let (c, y) = right.pop_last().unwrap();
                sum += c as i64 * y as i64;
                left.insert((c, y));
            }

            while left.len() > x {
                let (c, y) = left.pop_first().unwrap();
                sum -= c as i64 * y as i64;
                right.insert((c, y));
            }

            ans.push(sum);

            let y = nums[l];
            del(&mut cnt, &mut left, &mut right, y, &mut sum);
            *cnt.entry(y).or_default() -= 1;
            add(&mut cnt, &mut left, &mut right, y, &mut sum);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![6, 10, 12],
            Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![11, 15, 15, 15, 12],
            Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2)
        );
    }
}
