#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    long long beautifulSubarrays(vector<int> &nums) {
        unordered_map<int, int> m;
        m[0] = 1;
        int f = 0;
        auto ans = 0;
        for (int x : nums) {
            f ^= x;
            if (m.count(f)) {
                ans += m[f];
            }
            m[f] += 1;
        }
        return ans;
    }
};

int main() {
    vector<int> nums = {4, 3, 1, 2, 4};
    auto r = Solution().beautifulSubarrays(nums);
    assert(r == 2);

    nums = {1, 10, 4};
    r = Solution().beautifulSubarrays(nums);
    assert(r == 0);
    return 0;
}