#include <cstdio>
#include <vector>
#include <limits>

using namespace std;

class Solution {
public:
    int calculateMinimumHP(vector<vector<int>>& dungeon) {
        const int M = dungeon.size(), N = dungeon[0].size();
        vector<vector<int>> dp(M+1, vector<int>(N+1, numeric_limits<int>::max()));
        dp[M-1][N] = 1;
        dp[M][N-1] = 1;
        for (int i=M-1; i>=0; i--) {
            for (int j=N-1; j>=0; j--) {
                int n = min(dp[i+1][j], dp[i][j+1]) - dungeon[i][j];
                dp[i][j] = n<=0 ? 1 : n;
            }
        }
        return dp[0][0];
    }
};

int main() {
    vector<vector<int>> dungeon = {{1,-3,3},{0,-2,0},{-3,-3,-3}};
    printf("%d\n", Solution().calculateMinimumHP(dungeon));
    return 0;
}