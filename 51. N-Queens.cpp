#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    bool is_valid(const vector<string> &board, int n, int p, int q) {
        for (int k = 0; k < p; k++) {
            if (board[k][q] == 'Q') {
                return false;
            }
        }
        for (int i = p, j = q; i>=0 && j>=0; i--, j--) {
            if (board[i][j] == 'Q') {
                return false;
            }
        }
        for (int i = p, j = q; i>=0 && j<n; i--, j++) {
            if (board[i][j] == 'Q') {
                return false;
            }
        }
        return true;
    }

    void helper(vector<string>& board, const int n, const int row, vector<vector<string>>& r) {
        if (row == n) {
            r.push_back(board);
            return;
        }
        for (int j = 0; j < n; j++)
        {
            if (is_valid(board, n, row, j)) {
                board[row][j] = 'Q';
                helper(board, n, row+1, r);
                board[row][j] = '.';
            }
        }
    }

    vector<vector<string>> solveNQueens(int n) {
        vector<vector<string>> r;
        vector<string> board(n, string(n, '.'));
        helper(board, n, 0, r);
        return r;
    }
};

int main() {
    for (const auto &v : Solution().solveNQueens(8)) {
        for (const auto &s : v) {
            printf("%s\n", s.data());
        }
    }
    return 0;
}
