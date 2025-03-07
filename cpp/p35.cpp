#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    int searchInsert(vector<int> &nums, int target) {
        auto it = lower_bound(nums.begin(), nums.end(), target);
        return distance(nums.begin(), it);
    }
};

int main() {
    vector<int> nums = {1, 3, 5, 6};
    assert(2 == Solution().searchInsert(nums, 5));
    return 0;
}