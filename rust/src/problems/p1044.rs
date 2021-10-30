use std::collections::HashMap;

pub struct Solution;

const MODULUS: i64 = i32::MAX as i64;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        fn hash(s: &[u8]) -> i64 {
            s.iter().fold(0, |acc, b| (acc * 26 + *b as i64) % MODULUS)
        }

        fn is_valid(s: &[u8], k: usize) -> (bool, usize) {
            let m = (0..k).fold(1, |acc, _| (acc * 26) % MODULUS);
            let mut h = hash(&s[..k]);

            let mut sub_strs = HashMap::new();
            sub_strs.insert(h, &s[..k]);

            for i in 1..=(s.len() - k) {
                h = (h * 26 - (s[i - 1] as i64 * m) % MODULUS + MODULUS + s[i + k - 1] as i64)
                    % MODULUS;
                if let Some(sub) = sub_strs.get(&h) {
                    if *sub == &s[i..i + k] {
                        return (true, i);
                    }
                }
                sub_strs.insert(h, &s[i..i + k]);
            }

            (false, 0)
        }

        let mut left = 1;
        let mut right = s.len() - 1;
        let mut start = 0;

        let bytes = s.as_bytes().iter().map(|b| b - b'a').collect::<Vec<_>>();

        while left <= right {
            let mid = left + (right - left) / 2;
            let r = is_valid(&bytes, mid);
            if r.0 {
                start = r.1;
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
        }

        s[start..start + left - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "ana".to_string(),
            Solution::longest_dup_substring("banana".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "".to_string(),
            Solution::longest_dup_substring("abcd".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "knas".to_string(),
            Solution::longest_dup_substring("pmyiaxmicpmvqywlkisfzzutlxxjipitwcfxgqqfnxizowkqfmzsvkxryknasyvthozahbmapwqocupxcktmmtddxgatzftamrsvtddjpbnrojcqonmzxmknashplmykdbadiiccdkbyyzifqxvqfwgwihxgnrhqlmqprnjawuzcotutbkgnykuuwtzzzppmoyfmplhpznpnlwwbndekakpsmehzmbcfoyudgwsvehzgsfwqdkihiiwxfskicrbmoevxvpmmymihlkmgnuyohcofzfkehccwxezxypnnvqzrilr".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "okmzpmxzwjbfssktjtebhhxfphcxefhonkncnrumgduoaeltjvwqwydpdsrbxsgmcdxrthilniqxkqzuuqzqhlccmqcmccfqddncchadnthtxjruvwsmazlzhijygmtabbzelslebyrfpyyvcwnaiqkkzlyillxmkfggyfwgzhhvyzfvnltjfxskdarvugagmnrzomkhldgqtqnghsddgrjmuhpgkfcjkkkaywkzsikptkrvbnvuyamegwempuwfpaypmuhhpuqrufsgpiojhblbihbrpwxdxzolgqmzoyeblpvvrnbnsdnonhpmbrqissifpdavvscezqzclvukfgmrmbmmwvzfpxcgecyxneipexrzqgfwzdqeeqrugeiupukpveufmnceetilfsqjprcygitjefwgcvqlsxrasvxkifeasofcdvhvrpmxvjevupqtgqfgkqjmhtkyfsjkrdczmnettzdxcqexenpxbsharuapjmdvmfygeytyqfcqigrovhzbxqxidjzxfbrlpjxibtbndgubwgihdzwoywqxegvxvdgaoarlauurxpwmxqjkidwmfuuhcqtljsvruinflvkyiiuwiiveplnxlviszwkjrvyxijqrulchzkerbdyrdhecyhscuojbecgokythwwdulgnfwvdptzdvgamoublzxdxsogqpunbtoixfnkgbdrgknvcydmphuaxqpsofmylyijpzhbqsxryqusjnqfikvoikwthrmdwrwqzrdmlugfglmlngjhpspvnfddqsvrajviel"
.to_string(),
            Solution::longest_dup_substring("okmzpmxzwjbfssktjtebhhxfphcxefhonkncnrumgduoaeltjvwqwydpdsrbxsgmcdxrthilniqxkqzuuqzqhlccmqcmccfqddncchadnthtxjruvwsmazlzhijygmtabbzelslebyrfpyyvcwnaiqkkzlyillxmkfggyfwgzhhvyzfvnltjfxskdarvugagmnrzomkhldgqtqnghsddgrjmuhpgkfcjkkkaywkzsikptkrvbnvuyamegwempuwfpaypmuhhpuqrufsgpiojhblbihbrpwxdxzolgqmzoyeblpvvrnbnsdnonhpmbrqissifpdavvscezqzclvukfgmrmbmmwvzfpxcgecyxneipexrzqgfwzdqeeqrugeiupukpveufmnceetilfsqjprcygitjefwgcvqlsxrasvxkifeasofcdvhvrpmxvjevupqtgqfgkqjmhtkyfsjkrdczmnettzdxcqexenpxbsharuapjmdvmfygeytyqfcqigrovhzbxqxidjzxfbrlpjxibtbndgubwgihdzwoywqxegvxvdgaoarlauurxpwmxqjkidwmfuuhcqtljsvruinflvkyiiuwiiveplnxlviszwkjrvyxijqrulchzkerbdyrdhecyhscuojbecgokythwwdulgnfwvdptzdvgamoublzxdxsogqpunbtoixfnkgbdrgknvcydmphuaxqpsofmylyijpzhbqsxryqusjnqfikvoikwthrmdwrwqzrdmlugfglmlngjhpspvnfddqsvrajvielokmzpmxzwjbfssktjtebhhxfphcxefhonkncnrumgduoaeltjvwqwydpdsrbxsgmcdxrthilniqxkqzuuqzqhlccmqcmccfqddncchadnthtxjruvwsmazlzhijygmtabbzelslebyrfpyyvcwnaiqkkzlyillxmkfggyfwgzhhvyzfvnltjfxskdarvugagmnrzomkhldgqtqnghsddgrjmuhpgkfcjkkkaywkzsikptkrvbnvuyamegwempuwfpaypmuhhpuqrufsgpiojhblbihbrpwxdxzolgqmzoyeblpvvrnbnsdnonhpmbrqissifpdavvscezqzclvukfgmrmbmmwvzfpxcgecyxneipexrzqgfwzdqeeqrugeiupukpveufmnceetilfsqjprcygitjefwgcvqlsxrasvxkifeasofcdvhvrpmxvjevupqtgqfgkqjmhtkyfsjkrdczmnettzdxcqexenpxbsharuapjmdvmfygeytyqfcqigrovhzbxqxidjzxfbrlpjxibtbndgubwgihdzwoywqxegvxvdgaoarlauurxpwmxqjkidwmfuuhcqtljsvruinflvkyiiuwiiveplnxlviszwkjrvyxijqrulchzkerbdyrdhecyhscuojbecgokythwwdulgnfwvdptzdvgamoublzxdxsogqpunbtoixfnkgbdrgknvcydmphuaxqpsofmylyijpzhbqsxryqusjnqfikvoikwthrmdwrwqzrdmlugfglmlngjhpspvnfddqsvrajviel".to_string())
        );
    }
}
