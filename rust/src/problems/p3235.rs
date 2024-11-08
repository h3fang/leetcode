pub struct Solution;

fn is_two_circles_intersect(a: &[i32], b: &[i32]) -> bool {
    let (dx, dy, r) = (
        (a[0] - b[0]) as i64,
        (a[1] - b[1]) as i64,
        (a[2] + b[2]) as i64,
    );
    dx * dx + dy * dy <= r * r
}

fn is_point_in_circle((px, py): (i64, i64), (x, y, r): (i64, i64, i64)) -> bool {
    let (dx, dy) = (px - x, py - y);
    dx * dx + dy * dy <= r * r
}

fn is_circle_intersect_top_left((x, y, r): (i64, i64, i64), x_corner: i64, y_corner: i64) -> bool {
    (x <= x_corner && (y - y_corner).abs() <= r)
        || (y <= y_corner && x <= r)
        || (y > y_corner && is_point_in_circle((0, y_corner), (x, y, r)))
}

fn is_circle_intersect_bottom_right(
    (x, y, r): (i64, i64, i64),
    x_corner: i64,
    y_corner: i64,
) -> bool {
    (y <= y_corner && (x - x_corner).abs() <= r)
        || (x <= x_corner && y <= r)
        || (x > x_corner && is_point_in_circle((x_corner, 0), (x, y, r)))
}

impl Solution {
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        fn dfs(
            i: usize,
            circles: &[Vec<i32>],
            x_corner: i64,
            y_corner: i64,
            visited: &mut [bool],
        ) -> bool {
            let a = &circles[i];
            let (x1, y1, r1) = (a[0] as i64, a[1] as i64, a[2] as i64);
            if is_circle_intersect_bottom_right((x1, y1, r1), x_corner, y_corner) {
                return true;
            }
            visited[i] = true;
            for (j, b) in circles.iter().enumerate() {
                if visited[j] || !is_two_circles_intersect(a, b) {
                    continue;
                }
                let (x2, y2, r2) = (b[0] as i64, b[1] as i64, b[2] as i64);
                if x1 * r2 + x2 * r1 < (r1 + r2) * x_corner
                    && y1 * r2 + y2 * r1 < (r1 + r2) * y_corner
                    && dfs(j, circles, x_corner, y_corner, visited)
                {
                    return true;
                }
            }
            false
        }

        let (x_corner, y_corner) = (x_corner as i64, y_corner as i64);
        let mut visited = vec![false; circles.len()];
        for (i, a) in circles.iter().enumerate() {
            let (x, y, r) = (a[0] as i64, a[1] as i64, a[2] as i64);
            if is_point_in_circle((0, 0), (x, y, r))
                || is_point_in_circle((x_corner, y_corner), (x, y, r))
            {
                return false;
            }
            if !visited[i]
                && is_circle_intersect_top_left((x, y, r), x_corner, y_corner)
                && dfs(i, &circles, x_corner, y_corner, &mut visited)
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let circles = [[2, 1, 1]].iter().map(|c| c.to_vec()).collect();
        assert!(Solution::can_reach_corner(3, 4, circles));
    }

    #[test]
    fn case2() {
        let circles = [[1, 1, 2]].iter().map(|c| c.to_vec()).collect();
        assert!(!Solution::can_reach_corner(3, 3, circles));
    }

    #[test]
    fn case3() {
        let circles = [[2, 1, 1], [1, 2, 1]].iter().map(|c| c.to_vec()).collect();
        assert!(!Solution::can_reach_corner(3, 3, circles));
    }

    #[test]
    fn case4() {
        let circles = [[5, 5, 1]].iter().map(|c| c.to_vec()).collect();
        assert!(Solution::can_reach_corner(4, 4, circles));
    }
}
