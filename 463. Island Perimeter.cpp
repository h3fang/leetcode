#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int islandPerimeter_slow(vector<vector<int>> &grid) {
        int r = 0;
        const int M = grid.size();
        if (M == 0) {
            return r;
        }
        const int N = grid[0].size();
        for (int i = 0; i < M; i++) {
            for (int j = 0; j < N; j++) {
                if (grid[i][j]) {
                    r += 4;
                    if (i + 1 < M && grid[i + 1][j]) {
                        r -= 2;
                    }
                    if (j + 1 < N && grid[i][j + 1]) {
                        r -= 2;
                    }
                }
            }
        }
        return r;
    }

    void recursive(vector<vector<int>> &grid, int & r, int i, int j, const int M, const int N) {
        if (i < 0 || j < 0 || i >= M || j >= N || grid[i][j] != 1) {
            return;
        }
        r += 4;
        grid[i][j] = -1;
        if (i + 1 < M && grid[i + 1][j] == 1) {
            r -= 2;
        }
        if (i - 1 >= 0 && grid[i - 1][j] == 1) {
            r -= 2;
        }
        if (j + 1 < N && grid[i][j + 1] == 1) {
            r -= 2;
        }
        if (j - 1 >= 0 && grid[i][j - 1] == 1) {
            r -= 2;
        }
        recursive(grid, r, i+1, j, M, N);
        recursive(grid, r, i-1, j, M, N);
        recursive(grid, r, i, j + 1, M, N);
        recursive(grid, r, i, j - 1, M, N);
    }
    int islandPerimeter(vector<vector<int>> &grid) {
        int r = 0;
        const int M = grid.size();
        if (M == 0) {
            return r;
        }
        const int N = grid[0].size();
        for (int i = 0; i < M; i++) {
            for (int j = 0; j < N; j++) {
                if (grid[i][j]) {
                    recursive(grid, r, i, j, M, N);
                    return r;
                }
            }
        }
        return r;
    }
};

int main() {
    vector<vector<int>> grid = {{0, 1, 0, 0}, {1, 1, 1, 0}, {0, 1, 0, 0}, {1, 1, 0, 0}};
    printf("%d\n", Solution().islandPerimeter(grid));
    return 0;
}