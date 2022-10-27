pub struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        fn get_pixel(img: &[Vec<i32>], i: i32, j: i32) -> i32 {
            let n = img.len() as i32;
            if i < 0 || j < 0 || i >= n || j >= n {
                0
            } else {
                img[i as usize][j as usize]
            }
        }

        fn overlap(img1: &[Vec<i32>], img2: &[Vec<i32>], di: i32, dj: i32) -> i32 {
            let mut result = 0;
            for (i, row) in img2.iter().enumerate() {
                for (j, &p) in row.iter().enumerate() {
                    if p == 1 && p == get_pixel(img1, i as i32 + di, j as i32 + dj) {
                        result += 1;
                    }
                }
            }
            result
        }

        let n = img1.len() as i32;
        let mut result = 0;
        for di in -(n - 1)..=(n - 1) {
            for dj in -(n - 1)..=(n - 1) {
                result = result.max(overlap(&img1, &img2, di, dj));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let img1 = [[1, 1, 0], [0, 1, 0], [0, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let img2 = [[0, 0, 0], [0, 1, 1], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::largest_overlap(img1, img2));
    }

    #[test]
    fn case2() {
        let img1 = [[1]].iter().map(|r| r.to_vec()).collect();
        let img2 = [[1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::largest_overlap(img1, img2));
    }

    #[test]
    fn case3() {
        let img1 = [[0]].iter().map(|r| r.to_vec()).collect();
        let img2 = [[0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::largest_overlap(img1, img2));
    }
}
