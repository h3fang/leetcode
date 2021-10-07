#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int dp = nums[0], r = dp;
        for (int i = 1; i < nums.size(); i++)
        {
            dp = max(nums[i], dp + nums[i]);
            r = max(r, dp);
        }
        return r;
    }
};

int main() {
    vector<int> nums = {-2,1,-3,4,-1,2,1,-5,4};
    printf("%d\n", Solution().maxSubArray(nums));
    return 0;
}
