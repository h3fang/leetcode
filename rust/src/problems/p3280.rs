pub struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let date = date
            .split('-')
            .map(|x| {
                let x = x.parse::<u16>().unwrap();
                format!("{x:b}")
            })
            .collect::<Vec<_>>();
        date.join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "100000100000-10-11101",
            Solution::convert_date_to_binary("2080-02-29".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "11101101100-1-1",
            Solution::convert_date_to_binary("1900-01-01".to_string())
        );
    }
}
