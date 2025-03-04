#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> productExceptSelf(vector<int> &nums) {
        int n = nums.size();
        vector<int> ans(n, 1);
        int prod = nums[0];
        for (int i = 1; i < n; i++) {
            ans[i] = prod;
            prod *= nums[i];
        }
        prod = nums[n - 1];
        for (int i = n - 2; i >= 0; i--) {
            ans[i] *= prod;
            prod *= nums[i];
        }
        return ans;
    }
};

int main() {
    vector<int> nums = {1, 2, 3, 4};
    vector<int> expected = {24, 12, 8, 6};
    auto r = Solution().productExceptSelf(nums);
    assert(r == expected);
    return 0;
}
