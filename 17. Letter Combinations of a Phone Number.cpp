#include <vector>
#include <map>
#include <algorithm>
#include <cstdio>

using namespace std;
class Solution {
public:
    map<char, string> mapping;
    vector<string> r;
    int N;

    void lc(int i, const string& digits, string& str) {
        if (i == N) {
            r.push_back(str);
            return;
        }
        for (const char s : mapping[digits[i]]) {
            str[i] = s;
            lc(i+1, digits, str);
        }
    }

    vector<string> letterCombinations(string digits) {
        r.clear();
        N = digits.size();
        mapping['2'] = "abc";
        mapping['3'] = "def";
        mapping['4'] = "ghi";
        mapping['5'] = "jkl";
        mapping['6'] = "mno";
        mapping['7'] = "pqrs";
        mapping['8'] = "tuv";
        mapping['9'] = "wxyz";

        string str = digits;
        lc(0, digits, str);
        return r;
    }
};

int main() {
    vector<string> inputs = {"23"};

    auto s = Solution();
    for (auto& i : inputs) {
        printf("%s\n", i.data());
        for (auto& e : s.letterCombinations(i)) {
            printf("%s ", e.data());
        }
    }
    return 0;
}