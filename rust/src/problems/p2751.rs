pub struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        mut healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let mut indices: Vec<usize> = (0..n).collect::<Vec<_>>();
        indices.sort_unstable_by_key(|&i| positions[i]);
        let d = directions.as_bytes();
        let mut right: Vec<usize> = Vec::with_capacity(indices.len());
        for i in indices {
            if d[i] == b'L' {
                while let Some(j) = right.pop() {
                    match healths[j].cmp(&healths[i]) {
                        std::cmp::Ordering::Less => {
                            healths[i] -= 1;
                            healths[j] = 0;
                        }
                        std::cmp::Ordering::Equal => {
                            healths[i] = 0;
                            healths[j] = 0;
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            healths[j] -= 1;
                            healths[i] = 0;
                            right.push(j);
                            break;
                        }
                    }
                }
            } else {
                right.push(i);
            }
        }
        healths.into_iter().filter(|&x| x > 0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2, 17, 9, 15, 10],
            Solution::survived_robots_healths(
                vec![5, 4, 3, 2, 1],
                vec![2, 17, 9, 15, 10],
                "RRRRR".to_string()
            )
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![14],
            Solution::survived_robots_healths(
                vec![3, 5, 2, 6],
                vec![10, 10, 15, 12],
                "RLRL".to_string()
            )
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::survived_robots_healths(
                vec![1, 2, 5, 6],
                vec![10, 10, 11, 11],
                "RLRL".to_string()
            )
        );
    }
}
