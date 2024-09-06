pub struct Solution;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
        while tx > sx && ty > sy {
            match tx.cmp(&ty) {
                std::cmp::Ordering::Less => ty %= tx,
                std::cmp::Ordering::Equal => return tx == sx && ty == sy,
                std::cmp::Ordering::Greater => tx %= ty,
            }
        }

        if tx == sx && ty == sy {
            true
        } else if tx == sx {
            ty > sy && (ty - sy) % tx == 0
        } else if ty == sy {
            tx > sx && (tx - sx) % ty == 0
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::reaching_points(1, 1, 3, 5));
    }

    #[test]
    fn case2() {
        assert!(!Solution::reaching_points(1, 1, 2, 2));
    }

    #[test]
    fn case3() {
        assert!(Solution::reaching_points(1, 1, 1, 1));
    }
}
