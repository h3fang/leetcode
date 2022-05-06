pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut st: Vec<(char, i32)> = vec![];
        for c in s.chars() {
            if !st.is_empty() && st.last().unwrap().0 == c {
                st.last_mut().unwrap().1 += 1;
                if st.last().unwrap().1 == k {
                    st.pop();
                }
            } else {
                st.push((c, 1));
            }
        }
        st.into_iter()
            .flat_map(|(t, n)| std::iter::repeat(t).take(n as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("abcd", Solution::remove_duplicates("abcd".into(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "aa",
            Solution::remove_duplicates("deeedbbcccbdaa".into(), 3)
        );
    }
}
