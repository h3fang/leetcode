#include <cmath>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(int k, vector<int> &prices) {
        const int N = prices.size();
        if (N <= 1) {
            return 0;
        }
        int max_profit = 0;
        if (2 * k >= N) {
            for (int i = 0; i < N - 1; i++) {
                if (prices[i + 1] > prices[i]) {
                    max_profit += prices[i + 1] - prices[i];
                }
            }
            return max_profit;
        }

        vector<vector<int>> dp(k + 1, vector<int>(N + 1));
        for (int j = 1; j <= k; j++) {
            int prev_max = dp[j - 1][0] - prices[0];
            for (int i = 1; i < N; i++) {
                dp[j][i] = max(dp[j][i - 1], prices[i] + prev_max);
                prev_max = max(prev_max, dp[j - 1][i] - prices[i]);
                max_profit = max(max_profit, dp[j][i]);
            }
        }

        return max_profit;
    }
};

int main() {
    vector<int> prices = {3, 3, 5, 0, 0, 3, 1, 4};
    const int k = 2;
    printf("%d\n", Solution().maxProfit(k, prices));
    return 0;
}