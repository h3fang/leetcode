pub struct Solution;

impl Solution {
    pub fn count_rectangles(mut rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        rectangles.sort_unstable();
        let y_max = points.iter().map(|p| p[1]).max().unwrap();
        let rs = (1..=y_max)
            .map(|y| {
                rectangles
                    .iter()
                    .filter(|r| r[1] >= y)
                    .map(|r| [r[0], r[1]])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        points
            .iter()
            .map(|p| {
                let r = &rs[p[1] as usize - 1];
                (r.len() - r.partition_point(|r| r[0] < p[0])) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let rectangles = vec![vec![1, 2], vec![2, 3], vec![2, 5]];
        let points = vec![vec![2, 1], vec![1, 4]];
        assert_eq!(vec![2, 1], Solution::count_rectangles(rectangles, points));
    }

    #[test]
    fn case2() {
        let rectangles = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let points = vec![vec![1, 3], vec![1, 1]];
        assert_eq!(vec![1, 3], Solution::count_rectangles(rectangles, points));
    }
}
