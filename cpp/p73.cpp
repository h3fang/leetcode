#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    void setZeroes(vector<vector<int>> &matrix) {
        int m = matrix.size(), n = matrix[0].size();
        bool zero_in_col0 = false;
        for (int i = 0; i < m; i++) {
            if (matrix[i][0] == 0) {
                zero_in_col0 = true;
            }
            for (int j = 1; j < n; j++) {
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for (int i = m - 1; i >= 0; i--) {
            for (int j = 1; j < n; j++) {
                if (matrix[i][0] == 0 || matrix[0][j] == 0) {
                    matrix[i][j] = 0;
                }
            }
            if (zero_in_col0) {
                matrix[i][0] = 0;
            }
        }
    }
};

int main() {
    vector<vector<int>> matrix = {{1, 1, 1}, {1, 0, 1}, {1, 1, 1}};
    vector<vector<int>> expected = {{1, 0, 1}, {0, 0, 0}, {1, 0, 1}};
    Solution().setZeroes(matrix);
    assert(matrix == expected);
    return 0;
}