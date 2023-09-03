pub struct Solution;

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut t: Vec<i32> = dist
            .into_iter()
            .zip(speed)
            .map(|(d, s)| (d - 1) / s + 1)
            .collect();

        t.sort_unstable();
        let n = t.len() as i32;
        for (i, t) in t.into_iter().enumerate() {
            if i as i32 >= t {
                return i as i32;
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]));
    }
}
