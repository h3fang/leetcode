pub struct Solution;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut grow = (0..grow_time.len()).collect::<Vec<_>>();
        grow.sort_unstable_by_key(|&i| -grow_time[i]);
        let mut t = 0;
        let mut curr = 0;
        for i in grow {
            curr += plant_time[i];
            t = t.max(curr + grow_time[i]);
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let plant_time = vec![1, 4, 3];
        let grow_time = vec![2, 3, 1];
        assert_eq!(9, Solution::earliest_full_bloom(plant_time, grow_time));
    }

    #[test]
    fn case2() {
        let plant_time = vec![1, 4, 3];
        let grow_time = vec![2, 3, 1];
        assert_eq!(9, Solution::earliest_full_bloom(plant_time, grow_time));
    }

    #[test]
    fn case3() {
        let plant_time = vec![1];
        let grow_time = vec![1];
        assert_eq!(2, Solution::earliest_full_bloom(plant_time, grow_time));
    }
}
