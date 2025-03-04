#include <cassert>
#include <climits>
#include <vector>
using namespace std;

class Solution {
public:
    int maxSubArray(vector<int> &nums) {
        int f = nums[0], ans = nums[0];
        for (int i = 1; i < nums.size(); i++) {
            f = max(nums[i], f + nums[i]);
            ans = max(ans, f);
        }
        return ans;
    }
};

int main() {
    vector<int> nums = {-2, 1, -3, 4, -1, 2, 1, -5, 4};
    int ans = Solution().maxSubArray(nums);
    assert(6 == ans);
    return 0;
}