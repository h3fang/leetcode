pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, (i32, i32)> = HashMap::new();
        let (mut pre, mut result) = (0, 0);
        for (i, &x) in arr.iter().enumerate() {
            let p = pre ^ x;
            if let Some(e) = m.get(&p) {
                result += (i as i32) * e.0 - e.1;
            }
            let e = m.entry(pre).or_default();
            e.0 += 1;
            e.1 += i as i32;
            pre = p;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_triplets(vec![2, 3, 1, 6, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_triplets(vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::count_triplets(vec![2, 3]));
    }

    #[test]
    fn case4() {
        assert_eq!(3, Solution::count_triplets(vec![1, 3, 5, 7, 9]));
    }

    #[test]
    fn case5() {
        assert_eq!(
            8,
            Solution::count_triplets(vec![7, 11, 12, 9, 5, 2, 7, 17, 22])
        );
    }
}
