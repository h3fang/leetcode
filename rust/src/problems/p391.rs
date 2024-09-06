use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut points: HashMap<(i32, i32), i32> = HashMap::new();
        let mut area = 0i64;
        for r in &rectangles {
            *points.entry((r[0], r[1])).or_default() += 1;
            *points.entry((r[0], r[3])).or_default() += 1;
            *points.entry((r[2], r[1])).or_default() += 1;
            *points.entry((r[2], r[3])).or_default() += 1;
            area += (r[2] - r[0]) as i64 * (r[3] - r[1]) as i64;
        }

        let mut corners = Vec::new();
        for (k, v) in points {
            if v == 1 {
                corners.push(k);
            } else if !(v == 2 || v == 4) {
                return false;
            }
        }

        if corners.len() != 4 {
            return false;
        }

        corners.sort_unstable();

        (corners[3].0 - corners[0].0) as i64 * (corners[3].1 - corners[0].1) as i64 == area
    }

    pub fn is_rectangle_cover_scan_line(mut rectangles: Vec<Vec<i32>>) -> bool {
        rectangles.sort_unstable();
        let n = rectangles.len();
        let mut i = 0;
        let mut q: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();

        // initialize scan lines
        let x_leftmost = rectangles[0][0];
        let mut prev_y = i32::MIN;
        while i < n {
            let r = &rectangles[i];
            if r[0] != x_leftmost {
                break;
            }
            if prev_y == i32::MIN || prev_y == r[1] {
                prev_y = r[3];
                q.push((-r[2], -r[1], -r[3]));
                i += 1;
            } else {
                return false;
            }
        }

        // scan
        while let Some(mut edge) = q.pop() {
            // leftmost consecutive wall
            while let Some(next) = q.peek() {
                if next.0 == edge.0 && next.1 == edge.2 {
                    edge.2 = next.2;
                    q.pop();
                } else {
                    break;
                }
            }

            // is done?
            if i == n && q.is_empty() {
                return true;
            }

            // is their a consecutive match?
            while i < n {
                let r = &rectangles[i];
                if r[0] == -edge.0 && r[1] == -edge.1 {
                    edge.1 = -r[3];
                    q.push((-r[2], -r[1], -r[3]));
                    i += 1;
                } else {
                    break;
                }
            }
            if edge.1 != edge.2 {
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
        let rectangles = [
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [3, 2, 4, 4],
            [1, 3, 2, 4],
            [2, 3, 3, 4],
        ];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(Solution::is_rectangle_cover_scan_line(rectangles.clone()));
        assert!(Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn case2() {
        let rectangles = [[1, 1, 2, 3], [1, 3, 2, 4], [3, 1, 4, 2], [3, 2, 4, 4]];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(!Solution::is_rectangle_cover_scan_line(rectangles.clone()));
        assert!(!Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn case3() {
        let rectangles = [[1, 1, 3, 3], [3, 1, 4, 2], [1, 3, 2, 4], [3, 2, 4, 4]];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(!Solution::is_rectangle_cover_scan_line(rectangles.clone()));
        assert!(!Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn case4() {
        let rectangles = [[1, 1, 3, 3], [3, 1, 4, 2], [1, 3, 2, 4], [2, 2, 4, 4]];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(!Solution::is_rectangle_cover_scan_line(rectangles.clone()));
        assert!(!Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn case5() {
        let rectangles = [
            [0, 0, 4, 1],
            [7, 0, 8, 2],
            [6, 2, 8, 3],
            [5, 1, 6, 3],
            [4, 0, 5, 1],
            [6, 0, 7, 2],
            [4, 2, 5, 3],
            [2, 1, 4, 3],
            [0, 1, 2, 2],
            [0, 2, 2, 3],
            [4, 1, 5, 2],
            [5, 0, 6, 1],
        ];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(Solution::is_rectangle_cover_scan_line(rectangles.clone()));
        assert!(Solution::is_rectangle_cover(rectangles));
    }
}
