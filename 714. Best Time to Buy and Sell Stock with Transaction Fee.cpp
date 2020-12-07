#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int> &prices, int fee) {
        const int N = prices.size();
        if (N <= 1) {
            return 0;
		}
        int hold = -prices[0];
		int sold = 0;
        for (int i = 1; i < N; i++) {
            int prev_sold = sold;
			sold = max(sold, hold + prices[i] - fee);
			hold = max(hold, sold - prices[i]);
        }
		return sold;
    }
};

int main() {
    vector<int> prices = {1, 3, 2, 8, 4, 9};
    const int fee = 2;
    printf("%d\n", Solution().maxProfit(prices, fee));
    return 0;
}