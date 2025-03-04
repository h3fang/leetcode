#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    void rotate(vector<int> &nums, int k) {
        int n = nums.size();
        reverse(nums.begin(), nums.end());
        reverse(nums.begin(), nums.begin() + k);
        reverse(nums.begin() + k, nums.end());
    }
};

int main() {
    vector<int> nums = {1, 2, 3, 4, 5, 6, 7};
    vector<int> expected = {5, 6, 7, 1, 2, 3, 4};
    Solution().rotate(nums, 3);
    assert(nums == expected);
    return 0;
}
