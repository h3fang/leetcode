#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    int minPathSum(vector<vector<int>> &grid) {
        const int n = grid[0].size();
        vector<int> f = grid[0];
        for (int j = 1; j < n; j++) {
            f[j] += f[j - 1];
        }
        for (int i = 1; i < grid.size(); i++) {
            f[0] += grid[i][0];
            for (int j = 1; j < n; j++) {
                f[j] = grid[i][j] + min(f[j], f[j - 1]);
            }
        }
        return f[n - 1];
    }
};

int main() {
    vector<vector<int>> grid = {{1, 3, 1}, {1, 5, 1}, {4, 2, 1}};
    auto r = Solution().minPathSum(grid);
    assert(7 == r);

    grid = {{1, 2, 3}, {4, 5, 6}};
    r = Solution().minPathSum(grid);
    assert(12 == r);
    return 0;
}