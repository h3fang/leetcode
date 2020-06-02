#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int minDistance(string word1, string word2) {
        const int N1 = word1.size(), N2 = word2.size();
        vector<vector<int>> dp(N1 + 1, vector<int>(N2 + 1));
        for (int i = 0; i < N1 + 1; i++) {
            dp[i][0] = i;
        }
        for (int i = 0; i < N2 + 1; i++) {
            dp[0][i] = i;
        }
        for (int i = 1; i < N1 + 1; i++) {
            for (int j = 1; j < N2 + 1; j++) {
                if (word1[i - 1] == word2[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = min(min(dp[i - 1][j - 1] + 1, dp[i][j - 1] + 1), dp[i - 1][j] + 1);
                }
            }
        }
        return dp[N1][N2];
    }
};

int main() {
    string a = "horse", b = "ros";
    printf("%d\n", Solution().minDistance(a, b));
    return 0;
}