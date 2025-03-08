#include <algorithm>
#include <cassert>
#include <climits>
#include <vector>

using namespace std;

class Solution {
public:
    int lengthOfLIS(vector<int> &nums) {
        vector<int> f = {INT_MIN};
        for (int x : nums) {
            int i = distance(f.begin(), lower_bound(f.begin(), f.end(), x));
            if (i == f.size()) {
                f.push_back(x);
            } else {
                f[i] = x;
            }
        }
        return f.size() - 1;
    }
};

int main() {
    vector<int> nums = {10, 9, 2, 5, 3, 7, 101, 18};
    int r = Solution().lengthOfLIS(nums);
    assert(r == 4);

    nums = {0, 1, 0, 3, 2, 3};
    r = Solution().lengthOfLIS(nums);
    assert(r == 4);

    nums = {7, 7, 7, 7, 7, 7, 7};
    r = Solution().lengthOfLIS(nums);
    assert(r == 1);
    return 0;
}
