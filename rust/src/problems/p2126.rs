pub struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let max = *asteroids.iter().max().unwrap();
        let width = 32 - max.leading_zeros() as usize;
        let mut mins = vec![i32::MAX; width];
        let mut sums = vec![0; width];

        for x in asteroids {
            let w = 32 - x.leading_zeros() as usize - 1;
            mins[w] = mins[w].min(x);
            sums[w] += x as i64;
        }

        let mut m = mass as i64;
        for (min, sum) in mins.into_iter().zip(sums) {
            if min == i32::MAX {
                continue;
            }

            if m < min as i64 {
                return false;
            }

            m += sum;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]));
    }
}
