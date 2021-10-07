#include <cstdio>
#include <vector>

using namespace std;

class Solution {
    vector<bool> filled_colums, filled_fdiags, filled_bdiags;

public:
    bool is_valid(int n, int p, int q) {
        return !filled_colums[q] &&
               !filled_bdiags[p-q+n] &&
               !filled_fdiags[p+q];
    }

    void helper(const int n, const int row, int& r) {
        if (row == n) {
            r++;
            return;
        }
        for (int j = 0; j < n; j++)
        {
            if (is_valid(n, row, j)) {
                filled_colums[j] = true;
                filled_bdiags[row-j+n] = true;
                filled_fdiags[row+j] = true;
                helper(n, row+1, r);
                filled_colums[j] = false;
                filled_bdiags[row-j+n] = false;
                filled_fdiags[row+j] = false;
            }
        }
    }

    int totalNQueens(int n) {
        vector<vector<bool>> board(n, vector<bool>(n));
        filled_colums.resize(n);
        filled_bdiags.resize(2*n);
        filled_fdiags.resize(2*n);
        int r = 0;
        helper(n, 0, r);
        return r;
    }
};

int main() {
    printf("%d\n", Solution().totalNQueens(8));
    return 0;
}
