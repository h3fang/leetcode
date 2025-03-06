#include <cassert>
#include <deque>
#include <vector>

using namespace std;

class Solution {
public:
    int update(vector<vector<int>> &grid, deque<int> &fresh, int state) {
        const int m = grid.size(), n = grid[0].size();
        int s = fresh.size(), ans = 0;
        while (s > 0) {
            int k = fresh.front(), i = k / n, j = k % n;
            fresh.pop_front();
            if ((i - 1 >= 0 && grid[i - 1][j] == state) || (i + 1 < m && grid[i + 1][j] == state) ||
                (j - 1 >= 0 && grid[i][j - 1] == state) || (j + 1 < n && grid[i][j + 1] == state)) {
                grid[i][j] = state + 1;
                ans += 1;
            } else {
                fresh.push_back(k);
            }
            s--;
        }
        return ans;
    }

    int orangesRotting(vector<vector<int>> &grid) {
        const int m = grid.size(), n = grid[0].size();
        deque<int> fresh;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 1) {
                    fresh.push_back(i * n + j);
                }
            }
        }
        int days = 2;
        while (update(grid, fresh, days)) {
            days++;
        }
        return fresh.empty() ? days - 2 : -1;
    }
};

int main() {
    vector<vector<int>> grid = {{2, 1, 1}, {1, 1, 0}, {0, 1, 1}};
    assert(4 == Solution().orangesRotting(grid));
    return 0;
}