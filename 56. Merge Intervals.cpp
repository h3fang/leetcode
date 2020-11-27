#include <cstdio>
#include <algorithm>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        if (intervals.size() < 2) {
            return intervals;
        }
        sort(intervals.begin(), intervals.end(), [](const auto& l, const auto& r) {
            return l[0] < r[0];
        });
        vector<vector<int>> r;
        vector<int> prev = intervals[0];
        for (int i = 1; i < intervals.size(); i++) {
            if (intervals[i][0] <= prev[1]) {
                prev[1] = max(intervals[i][1], prev[1]);
            }
            else {
                r.push_back(prev);
                prev = intervals[i];
            }
        }
        r.push_back(prev);
        return r;
    }
};

int main() {
    vector<vector<int>> intervals = {{1,3},{2,6},{8,10},{15,18}};
    for (const auto& s : Solution().merge(intervals)) {
        printf("[");
        for (int i : s) {
            printf("%d,", i);
        }
        printf("\b], ");
    }
    printf("\b\b \n");
    return 0;
}
