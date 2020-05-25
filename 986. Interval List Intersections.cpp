#include <vector>
#include <queue>
#include <cstdio>

using namespace std;

class Solution {
public:
    vector<vector<int>> intervalIntersection(vector<vector<int>>& A, vector<vector<int>>& B) {
        vector<vector<int>> r;
        int i = 0, j = 0;
        while (i<A.size() && j<B.size()) {
            int g = max(A[i][0], B[j][0]), h = min(A[i][1], B[j][1]);
            if (g <= h) {
                r.push_back(vector<int>{g,h});
            }
            if (A[i][1] < B[j][1]) {i++;}
            else {j++;}
        }
        return r;
    }
};

int main() {
    // vector<vector<int>> A = {{0,2},{5,10},{13,23},{24,25}};
    // vector<vector<int>> B = {{1,5},{8,12},{15,24},{25,26}};
    vector<vector<int>> A = {{6,7},{8,13},{15,19}};
    vector<vector<int>> B = {{1,2},{4,5},{11,13},{15,16},{17,19}};
    for (auto& e : Solution().intervalIntersection(A, B)) {
        printf("[%d,%d] ", e[0], e[1]);
    }
    printf("\n");
    return 0;
}