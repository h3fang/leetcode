use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        fn check(
            i: usize,
            j: usize,
            points: &[Vec<i32>],
            checked: &mut HashSet<(usize, usize)>,
            curr_max: usize,
        ) -> usize {
            let mut result = 2;
            checked.insert((i, j));

            let p1 = &points[i];
            let p2 = &points[j];

            let dy = p2[1] - p1[1];
            let dx = p2[0] - p1[0];

            for (k, p3) in points.iter().enumerate().skip(j + 1) {
                if k == i || k == j {
                    continue;
                }
                if (result + points.len() - k) < curr_max {
                    break;
                }
                if (p3[1] - p2[1]) * dx == dy * (p3[0] - p2[0]) {
                    result += 1;
                    checked.insert((i, k));
                    checked.insert((j, k));
                }
            }
            result
        }

        let n = points.len();
        let mut result = 1;
        let mut checked = HashSet::new();
        for i in 0..n {
            for j in i + 1..n {
                if checked.contains(&(i, j)) {
                    continue;
                }
                let r = check(i, j, &points, &mut checked, result);
                result = result.max(r);
                if result > n / 2 {
                    return result as i32;
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[1, 1], [2, 2], [3, 3]];
        let points = points.into_iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(3, Solution::max_points(points));
    }

    #[test]
    fn case2() {
        let points = [[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
        let points = points.into_iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(4, Solution::max_points(points));
    }
}
