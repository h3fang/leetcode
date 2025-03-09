#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int minDistance(string word1, string word2) {
        const int N1 = word1.size(), N2 = word2.size();
        vector<int> dp(N2 + 1, 0);
        for (int i = 0; i < N2 + 1; i++) {
            dp[i] = i;
        }
        for (int i = 1; i < N1 + 1; i++) {
            dp[0] = i - 1;
            int pre = i - 1;
            for (int j = 1; j < N2 + 1; j++) {
                int temp = dp[j];
                if (word1[i - 1] == word2[j - 1]) {
                    dp[j] = pre;
                } else {
                    dp[j] = min(min(pre + 1, dp[j - 1] + 1), dp[j] + 1);
                }
                pre = temp;
            }
        }
        return dp[N2];
    }
};

int main() {
    assert(3 == Solution().minDistance("horse", "ros"));
    assert(5 == Solution().minDistance("intention", "execution"));
    return 0;
}