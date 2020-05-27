#include <vector>
#include <cstdio>

using namespace std;

class Solution {
public:
    int countSquares(vector<vector<int>>& matrix) {
        const int M = matrix.size(), N = matrix[0].size();
        int r = 0;

        for (int i=0; i<M; i++) {
            for (int j=0; j<N; j++) {
                if (matrix[i][j]) {
                    if (i > 0 && j > 0) {
                        matrix[i][j] = 1 + min(min(matrix[i-1][j], matrix[i][j-1]), matrix[i-1][j-1]);
                    }
                    r += matrix[i][j];
                }
            }
        }

        return r;
    }
};

int main() {
    vector<vector<int>> matrix = {{0,1,1,1},{1,1,1,1},{0,1,1,1}};
    printf("%d\n", Solution().countSquares(matrix));
    return 0;
}