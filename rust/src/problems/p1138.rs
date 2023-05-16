pub struct Solution;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let (mut cx, mut cy) = (0, 0);
        let mut result = String::new();
        for &c in target.as_bytes() {
            let c = (c - b'a') as i32;
            let nx = c / 5;
            let ny = c % 5;
            if nx < cx {
                result.push_str(&"U".repeat((cx - nx) as usize));
            }
            if ny < cy {
                result.push_str(&"L".repeat((cy - ny) as usize));
            }
            if nx > cx {
                result.push_str(&"D".repeat((nx - cx) as usize));
            }
            if ny > cy {
                result.push_str(&"R".repeat((ny - cy) as usize));
            }
            result.push('!');
            cx = nx;
            cy = ny;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    fn check(path: &str, target: &str) -> bool {
        let path = path.as_bytes();
        let mut i = 0;
        let (mut cx, mut cy) = (0, 0);
        for &t in target.as_bytes() {
            while i < path.len() {
                match path[i] {
                    b'U' => cx -= 1,
                    b'D' => cx += 1,
                    b'L' => cy -= 1,
                    b'R' => cy += 1,
                    b'!' => {
                        i += 1;
                        break;
                    }
                    _ => return false,
                }
                if cx < 0 || cx > 5 || cy < 0 || cy > 4 {
                    return false;
                }
                if cx == 5 && cy != 0 {
                    return false;
                }
                i += 1;
            }
            if t != b'a' + (cx * 5 + cy) as u8 {
                return false;
            }
        }
        true
    }

    #[test]
    fn case1() {
        let target = "leet".to_string();
        let result = Solution::alphabet_board_path(target.to_string());
        assert_eq!(15, result.len());
        assert!(check(result.as_str(), target.as_str()));
    }

    #[test]
    fn case2() {
        let target = "code".to_string();
        let result = Solution::alphabet_board_path(target.to_string());
        assert_eq!(14, result.len());
        assert!(check(result.as_str(), target.as_str()));
    }
}
