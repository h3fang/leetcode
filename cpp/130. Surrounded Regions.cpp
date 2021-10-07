#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    void dfs(vector<vector<char>>& board, int i, int j) {
        if (i < 0 || i > board.size() - 1 || j < 0 || j > board[0].size() -1) {
            return;
        }
        if (board[i][j] == 'O') {
            board[i][j] = 'A';
            dfs(board, i-1, j);
            dfs(board, i+1, j);
            dfs(board, i, j-1);
            dfs(board, i, j+1);
        }
    }

    void solve(vector<vector<char>>& board) {
        const int N = board.size();
        if (N < 3) { return; }
        const int M = board[0].size();
        if (M < 3) { return; }

        for (int k=0; k<M; k++) {
            dfs(board, 0, k);
            dfs(board, N-1, k);
        }

        for (int k=0; k<N; k++) {
            dfs(board, k, 0);
            dfs(board, k, M-1);
        }

        for (auto & row : board) {
            for (char& c : row) {
                if (c == 'O') {
                    c = 'X';
                } else if (c == 'A') {
                    c = 'O';
                }
            }
        }
    }
};

void printBorad(const vector<vector<char>>& board) {
    for (const auto & row : board) {
        for (const char c : row) {
            printf("%c ",  c);
        }
        printf("\n");
    }
    printf("\n");
}

int main() {
    vector<vector<char>> board = {{'X','O','X','O','X','O'},{'O','X','O','X','O','X'},{'X','O','X','O','X','O'},{'O','X','O','X','O','X'}};
    printBorad(board);
    Solution().solve(board);
    printBorad(board);
    return 0;
}