#include <cstdio>
#include <string>
#include <vector>

using namespace std;

// class Solution {
// public:
//     bool isMatch(string s, string p) {
//         const int M = s.size(), N = p.size();
//         int i = 0, j = 0, star_pos = -1, match_end = -1;
//         while (i < M) {
//             if (j < N && (s[i] == p[j] || p[j] == '?')) {
//                 i++;
//                 j++;
//             }
//             else if (j < N && p[j] == '*') {
//                 star_pos = j;
//                 j++;
//                 match_end = i;
//             }
//             else if (star_pos != -1) {
//                 j = star_pos + 1;
//                 match_end++;
//                 i = match_end;
//             }
//             else {
//                 return false;
//             }
//         }

//         while (p[j] == '*' & j < N) {
//             j++;
//         }

//         return j == N;
//     }
// };

// class Solution {
//     vector<vector<int>> dp;

// public:
//     bool match(const string& s, const string& p, int i, int j) {
//         if (dp[i][j] != -1) {
//             return dp[i][j] == 1;
//         }
//         bool r = false;
//         if (i == s.size() && j == p.size()) { r = true; }
//         else if (j == p.size()) { r = false; }
//         else if (i == s.size()) {
//             for (; j < p.size(); j++) {
//                 if (p[j] != '*') {
//                     r = false;
//                     break;
//                 }
//             }
//             if (j == p.size()) {
//                 r = true;
//             }
//         }
//         else if (p[j] == '*') {
//             r = match(s, p, i+1, j) || match(s, p, i, j+1);
//         }
//         else if (p[j] == '?' || p[j] == s[i]) {
//             r = match(s, p, i+1, j+1);
//         }
//         dp[i][j] = r ? 1 : 0;
//         return r;
//     }
//     bool isMatch(string s, string p) {
//         dp.resize(s.size() + 1);
//         for (auto& r : dp) {
//             r.resize(p.size() + 1, -1);
//         }
//         return match(s, p, 0, 0);
//     }
// };

class Solution {
public:
    bool isMatch(string s, string p) {
        vector<vector<bool>> dp;
        dp.resize(s.size() + 1);
        for (auto& r : dp) {
            r.resize(p.size() + 1, false);
        }
        for (int i = 0; i < s.size() + 1; i++) {
            for (int j = 0; j < p.size() + 1; j++) {
                if (i == 0 && j == 0) {
                    dp[i][j] = true;
                }
                else if (i == 0) {
                    dp[i][j] = p[j-1] == '*' && dp[i][j-1];
                }
                else if (j == 0) {
                    dp[i][j] = false;
                }
                else if (p[j-1] == '*') {
                    dp[i][j] = dp[i-1][j] || dp[i][j-1];
                }
                else if (s[i-1] == p[j-1] || p[j-1] == '?') {
                    dp[i][j] = dp[i-1][j-1];
                }
            }
        }
        return dp[s.size()][p.size()];
    }
};

int main() {
    string num1 = "aa";
    string num2 = "*";
    printf("%s\n", Solution().isMatch(num1, num2) ? "true" : "false");
    return 0;
}
