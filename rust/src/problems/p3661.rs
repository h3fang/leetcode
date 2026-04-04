pub struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, mut walls: Vec<i32>) -> i32 {
        let (m, n) = (walls.len(), robots.len());
        let mut rds = robots.into_iter().zip(distance).collect::<Vec<_>>();
        rds.push((0, 0));
        rds.push((i32::MAX, 0));
        rds.sort_unstable_by_key(|e| e.0);
        walls.sort_unstable();

        let mut f = [0; 2];

        let (mut left, mut curr, mut right) = (0, 0, [0; 2]);
        for (i, &(x, d)) in rds.iter().enumerate().skip(1).take(n) {
            let left_x = (x - d).max(rds[i - 1].0 + 1);
            while left < m && walls[left] < left_x {
                left += 1;
            }
            while curr < m && walls[curr] < x {
                curr += 1;
            }
            let curr1 = curr as i32;
            while curr < m && walls[curr] == x {
                curr += 1;
            }
            let left_walls = f[0] + curr as i32 - left as i32;
            for j in 0..2 {
                let (mut x2, d2) = rds[i + 1];
                if j == 0 {
                    x2 -= d2;
                }
                let right_x = (x + d).min(x2 - 1);
                while right[j] < m && walls[right[j]] <= right_x {
                    right[j] += 1;
                }
                f[j] = left_walls.max(f[1] + right[j] as i32 - curr1);
            }
        }

        f[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::max_walls(vec![4], vec![3], vec![1, 10]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]));
    }
}
