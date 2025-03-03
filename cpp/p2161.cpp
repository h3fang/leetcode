#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> pivotArray(vector<int> &nums, int pivot) {
        int n = nums.size(), l = 0, r = n - 1;
        vector<int> ans(n, pivot);
        for (int x : nums) {
            if (x < pivot) {
                ans[l] = x;
                l += 1;
            } else if (x > pivot) {
                ans[r] = x;
                r -= 1;
            }
        }
        for (int i = r + 1, j = nums.size() - 1; i < j; i += 1, j -= 1) {
            int t = ans[i];
            ans[i] = ans[j];
            ans[j] = t;
        }
        return ans;
    }
};

int main() {
    vector<int> nums = {9, 12, 5, 10, 14, 3, 10};
    auto r = Solution().pivotArray(nums, 10);
    vector<int> expected = {9, 5, 3, 10, 10, 12, 14};
    assert(r == expected);
    return 0;
}