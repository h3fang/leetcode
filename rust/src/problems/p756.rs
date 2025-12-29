pub struct Solution;

struct State {
    vis: Vec<bool>,
    pyramid: Vec<u32>,
    allowed: [[u8; 7]; 7],
}

fn dfs(s: &mut State, i: i32, j: i32) -> bool {
    if i == -1 {
        return true;
    }

    if s.vis[s.pyramid[i as usize] as usize] {
        return false;
    }

    if j == i + 1 {
        s.vis[s.pyramid[i as usize] as usize] = true;
        return dfs(s, i - 1, 0);
    }

    let b = s.pyramid[i as usize + 1];
    let mask = s.allowed[(b >> (j * 3)) as usize & 0b111][(b >> ((j + 1) * 3)) as usize & 0b111];
    for k in 1..=6 {
        if mask & (1 << k) > 0 {
            s.pyramid[i as usize] &= !(0b111 << (j * 3));
            s.pyramid[i as usize] |= k << (j * 3);
            if dfs(s, i, j + 1) {
                return true;
            }
        }
    }

    false
}

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let n = bottom.len();
        let mut pyramid = vec![0; n];
        for (i, b) in bottom.bytes().enumerate() {
            pyramid[n - 1] |= ((b - b'A' + 1) as u32) << (i * 3);
        }

        let mut al = [[0; 7]; 7];
        for a in allowed {
            let a = a.as_bytes();
            al[(a[0] - b'A' + 1) as usize][(a[1] - b'A' + 1) as usize] |= 1 << (a[2] - b'A' + 1);
        }

        let mut state = State {
            vis: vec![false; 1 << ((n - 1) * 3)],
            pyramid,
            allowed: al,
        };
        dfs(&mut state, n as i32 - 2, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let allowed = ["BCC", "CDE", "CEA", "FFF"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(Solution::pyramid_transition("BCD".to_string(), allowed));
    }

    #[test]
    fn case2() {
        let allowed = ["AAB", "AAC", "BCD", "BBE", "DEF"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(!Solution::pyramid_transition("AAAA".to_string(), allowed));
    }
}
