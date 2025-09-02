pub struct Solution;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| (p[0], -p[1]));
        let mut ans = 0;
        for (i, a) in points.iter().enumerate() {
            let mut max = i32::MIN;
            for b in &points[i + 1..] {
                if a[1] >= b[1] && b[1] > max {
                    ans += 1;
                    max = b[1];
                }
                if max == a[1] {
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[1, 1], [2, 2], [3, 3]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(0, Solution::number_of_pairs(points));
    }

    #[test]
    fn case2() {
        let points = [[6, 2], [4, 4], [2, 6]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::number_of_pairs(points));
    }

    #[test]
    fn case3() {
        let points = [[3, 1], [1, 3], [1, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::number_of_pairs(points));
    }
}
