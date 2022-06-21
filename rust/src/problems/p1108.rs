pub struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.', "[.]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("1[.]1[.]1[.]1", Solution::defang_i_paddr("1.1.1.1".into()));
    }
}
