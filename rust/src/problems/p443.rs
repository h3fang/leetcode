pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        chars.push('\n');
        let mut i = 0;
        let mut c = 1;
        let mut last = chars[0];
        for j in 1..chars.len() {
            if chars[j] != last {
                chars[i] = last;
                last = chars[j];
                i += 1;
                if c > 1 {
                    for ch in c.to_string().chars() {
                        chars[i] = ch;
                        i += 1;
                    }
                }
                c = 1;
            } else {
                c += 1;
            }
        }
        chars.pop();
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut chars = ["a", "a", "b", "b", "c", "c", "c"]
            .iter()
            .map(|s| s.chars().next().unwrap())
            .collect();
        let result = Solution::compress(&mut chars);
        println!("{chars:?}");
        assert_eq!(6, result);
        assert_eq!(&['a', '2', 'b', '2', 'c', '3'], &chars[..6]);
    }

    #[test]
    fn case2() {
        let mut chars = ["a"].iter().map(|s| s.chars().next().unwrap()).collect();
        let result = Solution::compress(&mut chars);
        println!("{chars:?}");
        assert_eq!(1, result);
        assert_eq!(&['a'], &chars[..1]);
    }

    #[test]
    fn case3() {
        let mut chars = [
            "a", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b",
        ]
        .iter()
        .map(|s| s.chars().next().unwrap())
        .collect();
        let result = Solution::compress(&mut chars);
        println!("{chars:?}");
        assert_eq!(4, result);
        assert_eq!(&['a', 'b', '1', '2'], &chars[..4]);
    }
}
