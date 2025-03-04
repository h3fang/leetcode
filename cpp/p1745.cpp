#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    bool checkPartitioning(string s) {
        int n = s.size();
        vector<vector<bool>> f(n, vector<bool>(n, false));
        for (int i = n - 1; i >= 0; i--) {
            f[i][i] = true;
            for (int j = i + 1; j < n; j++) {
                f[i][j] = s[i] == s[j] ? (j - i < 2 || f[i + 1][j - 1]) : false;
            }
        }
        vector<vector<bool>> g(3, vector<bool>(n, false));
        g[0][0] = true;
        for (int i = 0; i < 3; i++) {
            for (int j = 1; j < n; j++) {
                if (i == 0) {
                    g[i][j] = f[0][j];
                } else {
                    for (int k = 1; k <= j; k++) {
                        g[i][j] = g[i][j] || (g[i - 1][k - 1] && f[k][j]);
                    }
                }
            }
        }
        return g[2][n - 1];
    }
};

int main() {
    assert(Solution().checkPartitioning("abcbdd"));
    assert(!Solution().checkPartitioning("bcbddxy"));
    return 0;
}