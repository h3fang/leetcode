pub struct Solution;

impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort_unstable();
        heaters.sort_unstable();
        let n = heaters.len();
        let mut j = 0;
        let mut result = 0;
        for h in houses {
            while j < n - 1 && (heaters[j] - h).abs() >= (heaters[j + 1] - h).abs() {
                j += 1;
            }
            result = result.max((heaters[j] - h).abs());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::find_radius(vec![1, 2, 3], vec![2]));
    }
}
