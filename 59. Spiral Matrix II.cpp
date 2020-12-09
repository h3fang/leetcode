#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> generateMatrix(int n) {
        vector<vector<int>> m(n, vector<int>(n));
        const vector<vector<int>> steps = {{0,1}, {1,0}, {0,-1}, {-1,0}};
        int i = 0, j = -1, k = 1, p = n, d = 0, c = 1;
        while (p > 0) {
            for (int q = 0; q < p; q++) {
                i += steps[d][0];
                j += steps[d][1];
                m[i][j] = c++;
            }
            d = (d+1) % 4;
            k++;
            if (k == 2) {
                k = 0;
                p -= 1;
            }
        }

        return m;
    }
};

int main() {
    auto m = Solution().generateMatrix(5);
    for (const auto &row : m) {
        for (const auto &n : row) {
            printf("%d\t", n);
        }
        printf("\b \n");
    }
    return 0;
}