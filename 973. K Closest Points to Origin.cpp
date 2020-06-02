#include <algorithm>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> kClosest(vector<vector<int>>& points, int K) {
        nth_element(points.begin(), points.begin()+K, points.end(), [](const auto& a, const auto& b){
            return a[0]*a[0]+a[1]*a[1] < b[0]*b[0]+b[1]*b[1];
        });
        return vector<vector<int>>(points.begin(), points.begin()+K);
    }
};

int main() {
    const int N = 2;
    vector<vector<int>> points = {{1,0}, {1,2}, {0,2}};
    for (auto p : Solution().kClosest(points, N)) {
        printf("[%d, %d] ", p[0], p[1]);
    }
    printf("\n");
    return 0;
}