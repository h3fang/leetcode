pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        fn check(nodes: &[&str], i: &mut usize) -> bool {
            if *i >= nodes.len() {
                return false;
            }
            if nodes[*i] == "#" {
                *i += 1;
                return true;
            }
            *i += 1;
            check(nodes, i) && check(nodes, i)
        }
        let nodes = preorder.split(',').collect::<Vec<_>>();
        let mut i = 0;
        check(&nodes, &mut i) && i == nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_valid_serialization(
            "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_valid_serialization("1,#".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_valid_serialization("9,#,#,1".to_string()));
    }
}
