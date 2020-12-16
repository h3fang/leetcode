#include <cstdio>
#include <vector>
#include <cmath>

using namespace std;

class Solution {
    vector<int> dp{0};
public:
    int numSquares2(int n) {
        int N = dp.size();
        while (N <= n) {
            int m = numeric_limits<int>::max();
            for (int j=1; j*j <= N; j++) {
                m = min(m, dp[N-j*j] + 1);
            }
            dp.push_back(m);
            N++;
        }
        return dp[n];
    }

    int numSquares(int n) {
        const int sqrt_n = int(sqrt(n));

        // 1
        if (sqrt_n * sqrt_n == n) {
            return 1;
        }

        // 2
        for (int i=0; i<=sqrt_n; i++) {
            int r = n - i*i;
            if (int(sqrt(r)) * int(sqrt(r)) == r) {
                return 2;
            }
        }

        // 4
        while (n % 4 == 0) {
            n /= 4;
        }
        if (n % 8 == 7) {
            return 4;
        }

        return 3;
    }
};

int main() {
    printf("%d\n", Solution().numSquares(5254));
    return 0;
}