#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    bool canJump(vector<int> &nums) {
        int f = 0, n = nums.size();
        for (int i = 0; i < n; i++) {
            if (i > f) {
                return false;
            }
            f = max(f, i + nums[i]);
            if (f >= n - 1) {
                return true;
            }
        }
        return false;
    }
};

int main() {
    vector<int> nums = {2, 3, 1, 1, 4};
    assert(Solution().canJump(nums));
    nums = {3, 2, 1, 0, 4};
    assert(!Solution().canJump(nums));
    return 0;
}
