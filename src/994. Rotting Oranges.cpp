#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int update(vector<vector<int>>& grid, int& remaining) {
        const int H = grid.size(), W = grid[0].size();
        vector<pair<int, int>> rotted;
        remaining = 0;
        for (int i = 0; i < H; i++) {
            for (int j = 0; j < W; j++) {
                if (grid[i][j] == 1) {
                    if ((i-1 >= 0 && grid[i-1][j] == 2) ||
                        (i+1 < H && grid[i+1][j] == 2) ||
                        (j-1 >= 0 && grid[i][j-1] == 2) ||
                        (j+1 < W && grid[i][j+1] == 2)) {
                        rotted.push_back({i,j});
                    }
                    else {
                        remaining++;
                    }
                }
            }
        }
        for (auto& p : rotted) {
            grid[p.first][p.second] = 2;
        }
        return rotted.size();
    }

    int orangesRotting(vector<vector<int>>& grid) {
        int days = 0, remaining = 0;
        while (true) {
            int rotted = update(grid, remaining);
            if (rotted == 0) {
                break;
            }
            days++;
        }
        return remaining == 0 ? days : -1;
    }
};

int main() {
    vector<vector<int>> grid = {{2,1,1},{1,1,0},{0,1,1}};
    printf("%d\n", Solution().orangesRotting(grid));
    return 0;
}