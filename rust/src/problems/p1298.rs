pub struct Solution;

fn dfs(
    x: usize,
    candies: &[i32],
    keys: &[Vec<i32>],
    contained_boxes: &[Vec<i32>],
    has_key: &mut [i32],
    has_box: &mut [bool],
    ans: &mut i32,
) {
    *ans += candies[x];
    has_box[x] = false;

    for &y in &keys[x] {
        let y = y as usize;
        has_key[y] = 1;
        if has_box[y] {
            dfs(y, candies, keys, contained_boxes, has_key, has_box, ans);
        }
    }

    for &y in &contained_boxes[x] {
        let y = y as usize;
        has_box[y] = true;
        if has_key[y] == 1 {
            dfs(y, candies, keys, contained_boxes, has_key, has_box, ans);
        }
    }
}

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut has_key = status;
        let mut has_box = vec![false; has_key.len()];
        for &x in &initial_boxes {
            has_box[x as usize] = true;
        }

        let mut ans = 0;
        for x in initial_boxes {
            let x = x as usize;
            if has_box[x] && has_key[x] == 1 {
                dfs(
                    x,
                    &candies,
                    &keys,
                    &contained_boxes,
                    &mut has_key,
                    &mut has_box,
                    &mut ans,
                );
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
        let status = vec![1, 0, 1, 0];
        let candies = vec![7, 5, 4, 100];
        let keys = vec![vec![], vec![], vec![1], vec![]];
        let contained_boxes = vec![vec![1, 2], vec![3], vec![], vec![]];
        let initial_boxes = vec![0];
        assert_eq!(
            16,
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes)
        );
    }

    #[test]
    fn case2() {
        let status = vec![1, 0, 0, 0, 0, 0];
        let candies = vec![1, 1, 1, 1, 1, 1];
        let keys = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
        let contained_boxes = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
        let initial_boxes = vec![0];
        assert_eq!(
            6,
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes)
        );
    }

    #[test]
    fn case3() {
        let status = vec![1, 1, 1];
        let candies = vec![100, 1, 100];
        let keys = vec![vec![], vec![0, 2], vec![]];
        let contained_boxes = vec![vec![], vec![], vec![]];
        let initial_boxes = vec![1];
        assert_eq!(
            1,
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes)
        );
    }

    #[test]
    fn case4() {
        let status = vec![1];
        let candies = vec![100];
        let keys = vec![vec![]];
        let contained_boxes = vec![vec![]];
        let initial_boxes = vec![];
        assert_eq!(
            0,
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes)
        );
    }

    #[test]
    fn case5() {
        let status = vec![1, 1, 1];
        let candies = vec![2, 3, 2];
        let keys = vec![vec![], vec![], vec![]];
        let contained_boxes = vec![vec![], vec![], vec![]];
        let initial_boxes = vec![2, 1, 0];
        assert_eq!(
            7,
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes)
        );
    }
}
