#include <vector>
#include <queue>
#include <cstdio>

using namespace std;

class Solution {
public:
    int maxUncrossedLines(vector<int>& A, vector<int>& B) {
        const int M = A.size(), N = B.size();
        vector<vector<int>> dp(M+1, vector<int>(N+1, 0));
        for (int i=0; i<M; i++) {
            for (int j=0; j<N; j++) {
                if (A[i] == B[j]) {
                    dp[i+1][j+1] = 1 + dp[i][j];
                }
                else {
                    dp[i+1][j+1] = max(dp[i][j+1], dp[i+1][j]);
                }
            }
        }
        return dp[M][N];
    }
};

int main() {
    // vector<int> A = {2,5,1,2,5}, B = {10,5,2,1,5,2};
    vector<int> A = {1,3,7,1,7,5}, B = {1,9,2,5,1};
    printf("%d\n", Solution().maxUncrossedLines(A, B));
    return 0;
}