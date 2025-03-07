#include <algorithm>
#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
    int n;
    vector<vector<string>> r;

public:
    bool is_valid(vector<int> &queens, int col) {
        int row = queens.size(), i2 = row + col, j2 = row - col;
        for (int i = 0; i < queens.size(); i++) {
            int j = queens[i];
            int i1 = i + j, j1 = i - j;
            if (j == col || i1 == i2 || j1 == j2) {
                return false;
            }
        }
        return true;
    }

    void helper(const int i, vector<int> &queens) {
        if (i == n) {
            vector<string> board(n, string(n, '.'));
            for (int i = 0; i < n; i++) {
                board[i][queens[i]] = 'Q';
            }
            r.push_back(board);
            return;
        }
        for (int j = 0; j < n; j++) {
            if (is_valid(queens, j)) {
                queens.push_back(j);
                helper(i + 1, queens);
                queens.pop_back();
            }
        }
    }

    vector<vector<string>> solveNQueens(int n) {
        this->n = n;
        vector<int> queens;
        queens.reserve(n);
        helper(0, queens);
        return r;
    }
};

int main() {
    vector<vector<string>> expected = {{".Q..", "...Q", "Q...", "..Q."},
                                       {"..Q.", "Q...", "...Q", ".Q.."}};
    sort(expected.begin(), expected.end());
    auto r = Solution().solveNQueens(4);
    sort(r.begin(), r.end());
    assert(r == expected);
    return 0;
}
