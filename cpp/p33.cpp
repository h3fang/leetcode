#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    int search(vector<int> &nums, int target) {
        int a = 0, b = nums.size() - 1;
        while (a <= b) {
            int m = a + (b - a) / 2;
            if (nums[m] == target) {
                return m;
            }
            if (nums[a] <= nums[m]) {
                if (nums[a] <= target && target < nums[m]) {
                    b = m - 1;
                } else {
                    a = m + 1;
                }
            } else {
                if (nums[m] < target && target <= nums[b]) {
                    a = m + 1;
                } else {
                    b = m - 1;
                }
            }
        }
        return -1;
    }
};

int main() {
    vector<int> nums = {4, 5, 6, 7, 1, 2, 3};
    const int target = 1;
    assert(4 == Solution().search(nums, target));
    return 0;
}