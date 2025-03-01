#include <algorithm>
#include <cassert>
#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    void dfs(const string &s, const int start, vector<string> &current_list,
             vector<vector<string>> &r, vector<vector<bool>> &dp) {
        if (start >= s.size()) {
            r.push_back(current_list);
            return;
        }
        for (int end = start; end < s.size(); end++) {
            if (dp[start][end]) {
                current_list.push_back(s.substr(start, end - start + 1));
                dfs(s, end + 1, current_list, r, dp);
                current_list.pop_back();
            }
        }
    }

    vector<vector<string>> partition(string s) {
        vector<vector<string>> r;
        vector<string> current_list;
        int n = s.size();
        vector<vector<bool>> dp(n, vector<bool>(n, false));
        for (int i = n - 1; i >= 0; i--) {
            for (int j = i; j < n; j++) {
                dp[i][j] = s[i] == s[j] && (j - i < 2 || dp[i + 1][j - 1]);
            }
        }
        dfs(s, 0, current_list, r, dp);
        return r;
    }
};

int main() {
    auto r = Solution().partition("aab");
    sort(r.begin(), r.end());
    vector<vector<string>> expected = {{"a", "a", "b"}, {"aa", "b"}};
    assert(r == expected);
    return 0;
}