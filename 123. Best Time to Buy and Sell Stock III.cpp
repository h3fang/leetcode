#include <cstdio>
#include <vector>
#include <cmath>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int> &prices) {
        if (prices.size() <= 1) {
            return 0;
        }
        vector<vector<int>> dp(3, vector<int>(prices.size()+1));
		int max_profit = 0;
        for (int k = 1; k <= 2; k++) {
            int prev_max = dp[k-1][0] - prices[0];
            for (int i = 1; i < prices.size(); i++) {
                dp[k][i] = max(dp[k][i-1], prices[i] + prev_max);
                prev_max = max(prev_max, dp[k-1][i] - prices[i]);
                max_profit = max(max_profit, dp[k][i]);
            }
        }

        return max_profit;
    }
};

int main() {
    vector<int> prices = {3, 3, 5, 0, 0, 3, 1, 4};
    printf("%d\n", Solution().maxProfit(prices));
    return 0;
}