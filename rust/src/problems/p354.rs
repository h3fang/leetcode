pub struct Solution;

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut f = vec![0; envelopes.len() + 1];
        f[0] = envelopes[0][1];
        let mut len = 1;
        for e in envelopes.iter().skip(1) {
            if e[1] > f[len - 1] {
                f[len] = e[1];
                len += 1;
            } else {
                let mut left = 0;
                let mut right = len as i32 - 1;
                while left < right {
                    let mid = (left + right) / 2;
                    match f[mid as usize].cmp(&e[1]) {
                        std::cmp::Ordering::Less => {
                            left = mid + 1;
                        }
                        _ => {
                            right = mid;
                        }
                    }
                }
                if (left as usize) < len {
                    f[left as usize] = e[1];
                }
            }
        }
        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_envelopes(env: &[[i32; 2]]) -> Vec<Vec<i32>> {
        env.iter().map(|e| e.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let envelopes = parse_envelopes(&[[5, 4], [6, 4], [6, 7], [2, 3]]);
        assert_eq!(3, Solution::max_envelopes(envelopes));
    }

    #[test]
    fn case2() {
        let envelopes = parse_envelopes(&[[5, 4], [5, 4], [5, 4]]);
        assert_eq!(1, Solution::max_envelopes(envelopes));
    }
}
