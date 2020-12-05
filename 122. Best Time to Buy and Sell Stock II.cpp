#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
		const int N = prices.size();
        int buy_in = -1, profit = 0;
		for (int i = 0; i < N; i++) {
			if (i + 1 < N && prices[i+1] > prices[i]) {
				profit += prices[i+1] - prices[i];
			}
		}
		return profit;
    }
};


int main() {
    vector<int> prices = {7,1,5,3,6,4};
    printf("%d\n", Solution().maxProfit(prices));
    return 0;
}