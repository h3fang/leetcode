#include "helpers.h"
#include <cassert>
#include <vector>

using namespace std;

const int DIRS[4][2] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

class Solution {
public:
    int numIslands(vector<vector<char>> &grid) {
        int ans = 0, m = grid.size(), n = grid[0].size();
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] != '1') {
                    continue;
                }
                ans += 1;
                vector<int> q = {i * n + j};
                while (!q.empty()) {
                    int k = q.back(), i = k / n, j = k % n;
                    q.pop_back();
                    grid[i][j] = '#';
                    for (auto &d : DIRS) {
                        int i1 = i + d[0], j1 = j + d[1];
                        if (i1 < 0 || j1 < 0 || i1 == m || j1 == n || grid[i1][j1] != '1') {
                            continue;
                        }
                        q.push_back(i1 * n + j1);
                    }
                }
            }
        }
        return ans;
    }
};

int main() {
    vector<vector<char>> grid = {{'1', '1', '1', '1', '0'},
                                 {'1', '1', '0', '1', '0'},
                                 {'1', '1', '0', '0', '0'},
                                 {'0', '0', '0', '0', '0'}};
    int r = Solution().numIslands(grid);
    assert(1 == r);
    return 0;
}