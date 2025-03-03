#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    int subarraySum(vector<int> &nums, int k) {
        unordered_map<int, int> m;
        int ans = 0, sum = 0;
        m[0] = 1;
        for (int i = 0; i < nums.size(); i++) {
            sum += nums[i];
            ans += m[sum - k];
            m[sum] += 1;
        }
        return ans;
    }
};

int main() {
    vector<int> nums = {1, 1, 1};
    auto r = Solution().subarraySum(nums, 2);
    assert(r == 2);
    return 0;
}