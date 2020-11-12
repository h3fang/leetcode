#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> flipAndInvertImage(vector<vector<int>>& A) {
        const int N = A.size();
        for (auto & row : A) {
            for (int i = 0; i < (N+1)/2; i++) {
                if (row[i] == row[N-i-1]) {
                    row[i] = row[i] == 1 ? 0 : 1;
                    row[N-i-1] = row[i];
                }
            }
        }
        return A;
    }
};

int main() {
    vector<vector<int>> input = {{1,1,0},{1,0,1},{0,0,0}};
    auto r = Solution().flipAndInvertImage(input);
    for (const auto & row : r) {
        for (int e : row) {
            printf("%d,", e);
        }
        printf("\b \n");
    }
    return 0;
}