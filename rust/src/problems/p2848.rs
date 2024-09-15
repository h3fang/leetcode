pub struct Solution;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut points = [false; 101];
        for c in nums {
            for i in c[0]..=c[1] {
                points[i as usize] = true;
            }
        }
        points.iter().filter(|&&x| x).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [[3, 6], [1, 5], [4, 7]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(7, Solution::number_of_points(nums));
    }

    #[test]
    fn case2() {
        let nums = [[1, 3], [5, 8]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(7, Solution::number_of_points(nums));
    }
}
