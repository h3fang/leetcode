pub struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        fn area(p1: &[i32], p2: &[i32], p3: &[i32]) -> f64 {
            0.5 * (p1[0] * p2[1] + p2[0] * p3[1] + p3[0] * p1[1]
                - p1[0] * p3[1]
                - p2[0] * p1[1]
                - p3[0] * p2[1])
                .abs() as f64
        }

        fn cross(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
            (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0])
        }

        fn convex_hull(mut points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            points.sort_unstable();
            let mut used = vec![false; points.len()];
            let mut hull = vec![0];
            for i in 1..points.len() {
                while hull.len() > 1
                    && cross(
                        &points[hull[hull.len() - 2]],
                        &points[hull[hull.len() - 1]],
                        &points[i],
                    ) < 0
                {
                    let p = hull.pop().unwrap();
                    used[p] = false;
                }
                hull.push(i);
                used[i] = true;
            }

            let m = hull.len();

            for i in (0..points.len()).rev() {
                if used[i] {
                    continue;
                }
                while hull.len() > m
                    && cross(
                        &points[hull[hull.len() - 2]],
                        &points[hull[hull.len() - 1]],
                        &points[i],
                    ) < 0
                {
                    hull.pop();
                }
                hull.push(i);
            }

            hull.pop();
            hull.into_iter()
                .map(|i| std::mem::take(&mut points[i]))
                .collect()
        }

        let hull = convex_hull(points);
        let n = hull.len();
        let mut result = 0.0f64;
        for i in 0..n - 2 {
            let mut k = i + 2;
            for j in i + 1..n - 1 {
                while k + 1 < n {
                    let curr = area(&hull[i], &hull[j], &hull[k]);
                    let next = area(&hull[i], &hull[j], &hull[k + 1]);
                    if next <= curr {
                        break;
                    }
                    k += 1;
                }
                let a = area(&hull[i], &hull[j], &hull[k]);
                result = result.max(a);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_points(pts: &[[i32; 2]]) -> Vec<Vec<i32>> {
        pts.iter().map(|p| p.to_vec()).collect()
    }

    fn assert_close(a: f64, b: f64) {
        println!("{} {}", a, b);
        assert!((a - b).abs() < 1e-6)
    }

    #[test]
    fn case1() {
        let points = parse_points(&[[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]]);
        assert_close(2.0, Solution::largest_triangle_area(points));
    }

    #[test]
    fn case2() {
        let points = parse_points(&[[9, 0], [0, 2], [3, 1], [10, 8]]);
        assert_close(37.0, Solution::largest_triangle_area(points));
    }
}
