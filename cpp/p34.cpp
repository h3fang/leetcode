#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> searchRange(vector<int> &nums, int target) {
        const int N = nums.size();
        if (N == 0) {
            return {-1, -1};
        }
        int i = distance(nums.begin(), upper_bound(nums.begin(), nums.end(), target - 1));
        if (i == nums.size() || nums[i] != target) {
            return {-1, -1};
        }
        int j = distance(nums.begin(), lower_bound(nums.begin(), nums.end(), target + 1));
        return {i, j - 1};
    }
};

int main() {
    vector<int> nums = {0, 1, 3, 4, 5, 5, 7};
    const int target = 5;
    auto r = Solution().searchRange(nums, target);
    vector<int> expected = {4, 5};
    assert(expected == r);
    return 0;
}