pub struct Solution;

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (mut min, mut max, mut d) = (0, 0, 0);
        for x in differences {
            d += x as i64;
            min = min.min(d);
            max = max.max(d);
        }
        (upper as i64 - lower as i64 + 1 - (max - min)).max(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::number_of_arrays(vec![1, -3, 4], 1, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::number_of_arrays(vec![4, -7, 2], 3, 6));
    }
}
