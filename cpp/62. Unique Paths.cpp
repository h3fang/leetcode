#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int uniquePaths(int m, int n) {
        vector<int> dp(n+1);
        dp[n-1] = 1;
        for (int i = m-1; i >= 0; i--) {
            for (int j = n-1; j >= 0; j--) {
                dp[j] += dp[j+1];
            }
        }
        return dp[0];
    }
};

int main() {
    printf("%d\n", Solution().uniquePaths(3,2));
    return 0;
}