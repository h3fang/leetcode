pub struct Solution;

use std::collections::BTreeMap;

#[derive(Default)]
struct Container {
    k: usize,
    st1: BTreeMap<i32, i32>,
    st2: BTreeMap<i32, i32>,
    pub sum: i64,
    size: usize,
}

impl Container {
    fn remove_one(map: &mut BTreeMap<i32, i32>, key: i32) -> bool {
        if let Some(e) = map.get_mut(&key) {
            if *e > 1 {
                *e -= 1;
            } else {
                map.remove(&key);
            }
            true
        } else {
            false
        }
    }

    fn add_one(map: &mut BTreeMap<i32, i32>, key: i32) {
        *map.entry(key).or_insert(0) += 1;
    }

    fn adjust(&mut self) {
        while self.size < self.k && !self.st2.is_empty() {
            if let Some(&x) = self.st2.keys().next() {
                Self::add_one(&mut self.st1, x);
                Self::remove_one(&mut self.st2, x);
                self.sum += x as i64;
                self.size += 1;
            }
        }

        while self.size > self.k {
            if let Some(&x) = self.st1.keys().last() {
                Self::add_one(&mut self.st2, x);
                Self::remove_one(&mut self.st1, x);
                self.sum -= x as i64;
                self.size -= 1;
            }
        }
    }

    fn add(&mut self, x: i32) {
        if !self.st2.is_empty() && x >= *self.st2.keys().next().unwrap() {
            *self.st2.entry(x).or_insert(0) += 1;
        } else {
            *self.st1.entry(x).or_insert(0) += 1;
            self.sum += x as i64;
            self.size += 1;
        }
        self.adjust();
    }

    fn erase(&mut self, x: i32) {
        if Self::remove_one(&mut self.st1, x) {
            self.sum -= x as i64;
            self.size -= 1;
        } else {
            Self::remove_one(&mut self.st2, x);
        }
        self.adjust();
    }
}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let (k, dist) = (k as usize, dist as usize);

        let mut cnt = Container {
            k: k - 2,
            ..Default::default()
        };
        for &x in &nums[1..k - 1] {
            cnt.add(x);
        }

        let mut ans = cnt.sum + nums[k - 1] as i64;
        for (i, &x) in nums.iter().enumerate().skip(k) {
            let j = i as i32 - dist as i32 - 1;
            if j > 0 {
                cnt.erase(nums[j as usize]);
            }
            cnt.add(nums[i - 1]);
            ans = ans.min(cnt.sum + x as i64);
        }

        ans + nums[0] as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(36, Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1));
    }
}
