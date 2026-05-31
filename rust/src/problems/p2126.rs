pub struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut m = mass as i64;
        asteroids.sort_unstable();
        for a in asteroids {
            if m < a as i64 {
                return false;
            }
            m += a as i64;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]));
    }
}
