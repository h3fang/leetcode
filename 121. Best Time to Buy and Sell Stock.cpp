#include <cstdio>
#include <vector>
#include <limits>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int max_profit = 0, min_price = numeric_limits<int>::max();
		for (int p : prices) {
			if (p < min_price) {
				min_price = p;
			}
			else if (p - min_price > max_profit) {
				max_profit = p - min_price;
			}
		}
		return max_profit;
    }
};


int main() {
    vector<int> prices = {3,3,5,0,0,3,1,4};
    printf("%d\n", Solution().maxProfit(prices));
    return 0;
}