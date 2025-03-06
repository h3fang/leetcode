#include <algorithm>
#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
    vector<string> ans;
    void bt(int n, int i, int left, string &s) {
        if (i == n) {
            ans.push_back(s);
            return;
        }
        if (left * 2 < n) {
            s.push_back('(');
            bt(n, i + 1, left + 1, s);
            s.pop_back();
        }
        if (i - left < left) {
            s.push_back(')');
            bt(n, i + 1, left, s);
            s.pop_back();
        }
    }

public:
    vector<string> generateParenthesis(int n) {
        string s;
        bt(2 * n, 0, 0, s);
        return ans;
    }
};

int main() {
    auto r = Solution().generateParenthesis(3);
    sort(r.begin(), r.end());
    vector<string> expected = {"((()))", "(()())", "(())()", "()(())", "()()()"};
    sort(expected.begin(), expected.end());
    assert(r == expected);
    return 0;
}