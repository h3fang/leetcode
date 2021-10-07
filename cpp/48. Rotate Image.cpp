#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    void rotate(vector<vector<int>>& matrix) {
        const int N = matrix.size();
        for (int i = 0; i < N; i++) {
            for (int j = i; j < N; j++) {
                swap(matrix[j][i], matrix[i][j]);
            }
        }

        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N / 2; j++) {
                swap(matrix[i][j], matrix[i][N-1-j]);
            }
        }
    }
};

int main() {
    vector<vector<int>> matrix = {{1,2,3},{4,5,6},{7,8,9}};
    Solution().rotate(matrix);
    for (const auto& row : matrix) {
        for (const auto& n : row) {
            printf("%d\t", n);
        }
        printf("\b \n");
    }
    return 0;
}