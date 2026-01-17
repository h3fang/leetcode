pub struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let rects = bottom_left
            .into_iter()
            .map(|v| (v[0], v[1]))
            .zip(top_right.into_iter().map(|v| (v[0], v[1])))
            .collect::<Vec<_>>();
        let mut max = 0;
        for (i, ((bx1, by1), (tx1, ty1))) in rects.iter().enumerate() {
            if tx1 - bx1 <= max || ty1 - by1 <= max {
                continue;
            }
            for ((bx2, by2), (tx2, ty2)) in &rects[i + 1..] {
                let w = tx1.min(tx2) - bx1.max(bx2);
                let h = ty1.min(ty2) - by1.max(by2);
                max = max.max(w.min(h));
            }
        }
        max as i64 * max as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let bottom_left = [[1, 1], [2, 2], [3, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let top_right = [[3, 3], [4, 4], [6, 6]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(1, Solution::largest_square_area(bottom_left, top_right));
    }

    #[test]
    fn case2() {
        let bottom_left = [[1, 1], [2, 2], [1, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let top_right = [[3, 3], [4, 4], [3, 4]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(1, Solution::largest_square_area(bottom_left, top_right));
    }

    #[test]
    fn case3() {
        let bottom_left = [[1, 1], [3, 3], [3, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let top_right = [[2, 2], [4, 4], [4, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(0, Solution::largest_square_area(bottom_left, top_right));
    }
}
