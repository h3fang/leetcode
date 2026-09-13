pub struct Solution;

fn convert(img: &[Vec<i32>]) -> Vec<i32> {
    img.iter()
        .map(|row| row.iter().fold(0, |acc, x| (acc << 1) | x))
        .collect()
}

fn overlap(img1: &[i32], img2: &[i32], di: i32, dj: i32) -> i32 {
    let mut ans = 0;
    let l1 = 0.max(-di) as usize;
    let l2 = 0.max(di) as usize;
    for (&r1, &r2) in img1[l1..].iter().zip(&img2[l2..]) {
        let r1 = if dj >= 0 { r1 >> dj } else { r1 << dj.abs() };
        ans += (r1 & r2).count_ones();
    }
    ans as i32
}

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len() as i32;
        let img1 = convert(&img1);
        let img2 = convert(&img2);
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
