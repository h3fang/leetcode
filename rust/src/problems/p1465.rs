pub struct Solution;

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        horizontal_cuts.sort_unstable();

        vertical_cuts.push(0);
        vertical_cuts.push(w);
        vertical_cuts.sort_unstable();
        let max_h = horizontal_cuts
            .windows(2)
            .map(|w| w[1] - w[0])
            .max()
            .unwrap();
        let max_w = vertical_cuts.windows(2).map(|w| w[1] - w[0]).max().unwrap();

        ((max_h as i64 * max_w as i64) % 10_0000_0007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![1, 2, 4];
        let vertical_cuts = vec![1, 3];
        assert_eq!(4, Solution::max_area(h, w, horizontal_cuts, vertical_cuts));
    }

    #[test]
    fn case2() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3, 1];
        let vertical_cuts = vec![1];
        assert_eq!(6, Solution::max_area(h, w, horizontal_cuts, vertical_cuts));
    }
}
