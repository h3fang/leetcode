pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut q: BinaryHeap<i32> = BinaryHeap::new();
        for (i, w) in heights.windows(2).enumerate() {
            if w[1] > w[0] {
                let delta = w[1] - w[0];
                if bricks >= delta {
                    bricks -= delta;
                    q.push(delta);
                } else if ladders > 0 {
                    match q.peek() {
                        Some(&n) if n <= delta => ladders -= 1,
                        Some(&n) => {
                            q.pop();
                            q.push(delta);
                            bricks += n - delta;
                            ladders -= 1;
                        }
                        None => ladders -= 1,
                    }
                } else {
                    return i as i32;
                }
            }
        }
        heights.len() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        let bricks = 5;
        let ladders = 1;
        assert_eq!(4, Solution::furthest_building(heights, bricks, ladders));
    }

    #[test]
    fn case2() {
        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        let bricks = 10;
        let ladders = 2;
        assert_eq!(7, Solution::furthest_building(heights, bricks, ladders));
    }

    #[test]
    fn case3() {
        let heights = vec![14, 3, 19, 3];
        let bricks = 17;
        let ladders = 0;
        assert_eq!(3, Solution::furthest_building(heights, bricks, ladders));
    }
}
