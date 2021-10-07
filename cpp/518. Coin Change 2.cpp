#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int change(int amount, vector<int>& coins) {
        vector<int> dp(amount+1); dp[0] = 1;
        for (int c : coins) {
            for (int i = c; i <= amount; i++) {
                dp[i] += dp[i - c];
            }
        }
        return dp[amount];
    }
};

int main() {
    const int amounts = 100;
    vector<int> coins = {2, 5, 10, 15, 20, 50};
    printf("%d\n", Solution().change(amounts, coins));
    return 0;
}