#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        vector<int> r;
        const vector<vector<int>> steps = {{0,1}, {1,0}, {0,-1}, {-1,0}};
        size_t ls[2] = {matrix[0].size(), matrix.size()};
        int i = 0, j = -1, k = 1, d = 0, vh = 0;
        while (ls[vh] > 0) {
            for (int q = 0; q < ls[vh]; q++) {
                i += steps[d][0];
                j += steps[d][1];
                r.push_back(matrix[i][j]);
            }
            d = (d+1) % 4;
            vh = (vh+1) % 2;
            ls[vh] -= 1;
        }

        return r;
    }
};

int main() {
    vector<vector<int>> matrix = {{1,2,3,4},{5,6,7,8},{9,10,11,12}};
    auto m = Solution().spiralOrder(matrix);
    for (int i : m) {
        printf("%d,", i);
    }
    printf("\b \n");
    return 0;
}