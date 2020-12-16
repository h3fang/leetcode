#include <cstdio>
#include <limits>
#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int> &prices) {
        const int N = prices.size();
        if (N <= 1) {
            return 0;
		}
        vector<vector<int>> dp(3, vector<int>(N));
		dp[0][0] = -prices[0];
		dp[1][0] = 0;
		dp[2][0] = numeric_limits<int>::min();
        for (int i = 1; i < N; i++) {
			dp[0][i] = max(dp[0][i-1], dp[1][i-1] - prices[i]);
			dp[1][i] = max(dp[1][i-1], dp[2][i-1]);
			dp[2][i] = dp[0][i-1] + prices[i];
        }
		return max(dp[1][N-1], dp[2][N-1]);
    }
};

int main() {
    vector<int> prices = {2, 1, 4};
    printf("%d\n", Solution().maxProfit(prices));
    return 0;
}