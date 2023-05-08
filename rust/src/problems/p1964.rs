pub struct Solution;

fn upper_bound(nums: &[i32], target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] > target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r as i32
}

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut f = Vec::with_capacity(obstacles.len());
        obstacles
            .into_iter()
            .map(|s| {
                if f.is_empty() || s >= *f.last().unwrap() {
                    f.push(s);
                    f.len() as i32
                } else {
                    let i = upper_bound(&f, s);
                    f[i as usize] = s;
                    i + 1
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 2, 3, 3],
            Solution::longest_obstacle_course_at_each_position(vec![1, 2, 3, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 2, 1],
            Solution::longest_obstacle_course_at_each_position(vec![2, 2, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![1, 1, 2, 3, 2, 2],
            Solution::longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2])
        );
    }
}
