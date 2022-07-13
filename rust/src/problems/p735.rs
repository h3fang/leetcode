pub struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut s: Vec<i32> = Vec::with_capacity(asteroids.len());
        for a in asteroids {
            let mut canceled = false;
            if a < 0 {
                while let Some(&b) = s.last() {
                    if b > 0 {
                        match b.cmp(&(-a)) {
                            std::cmp::Ordering::Less => {
                                s.pop();
                            }
                            std::cmp::Ordering::Equal => {
                                s.pop();
                                canceled = true;
                                break;
                            }
                            std::cmp::Ordering::Greater => {
                                canceled = true;
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            if !canceled {
                s.push(a);
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![5, 10], Solution::asteroid_collision(vec![5, 10, -5]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0; 0], Solution::asteroid_collision(vec![8, -8]));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![10], Solution::asteroid_collision(vec![10, 2, -5]));
    }
}
