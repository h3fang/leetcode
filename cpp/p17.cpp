#include <cassert>
#include <string>
#include <vector>

using namespace std;

const vector<string> mapping = {"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};

class Solution {
public:
    vector<string> r;

    void dfs(int i, const string &digits, string &str) {
        if (i == digits.size()) {
            r.push_back(str);
            return;
        }
        for (const char s : mapping[digits[i] - '0']) {
            str[i] = s;
            dfs(i + 1, digits, str);
        }
    }

    vector<string> letterCombinations(string digits) {
        if (digits.empty()) {
            return r;
        }
        string str = digits;
        dfs(0, digits, str);
        return r;
    }
};

int main() {
    auto s = Solution().letterCombinations("23");
    vector<string> expected = {"ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"};
    assert(s == expected);
    return 0;
}