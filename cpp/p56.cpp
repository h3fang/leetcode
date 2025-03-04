#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>> &intervals) {
        sort(intervals.begin(), intervals.end(), [](auto &a, auto &b) {
            if (a[0] == b[0]) {
                return a[1] > b[1];
            } else {
                return a[0] < b[0];
            }
        });
        vector<vector<int>> ans;
        ans.reserve(intervals.size());
        for (auto v : intervals) {
            if (ans.empty() || ans.back()[1] < v[0]) {
                ans.push_back(v);
                continue;
            }
            auto &b = ans.back();
            b[1] = max(b[1], v[1]);
        }
        return ans;
    }
};

int main() {
    vector<vector<int>> intervals = {{1, 3}, {2, 6}, {8, 10}, {15, 18}};
    vector<vector<int>> expected = {{1, 6}, {8, 10}, {15, 18}};
    auto r = Solution().merge(intervals);
    assert(r == expected);
    return 0;
}
