#include <cstdio>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    void nextPermutation(vector<int>& nums) {
        int i = nums.size()-1;
        for (; i > 0; i--) {
            if (nums[i-1] < nums[i]) {
                int k = i;
                for (int j = k; j < nums.size(); j++) {
                    if (nums[j] <= nums[k] && nums[j] > nums[i-1]) {
                        k = j;
                    }
                }
                swap(nums[i-1], nums[k]);
                reverse(nums.begin()+i, nums.end());
                break;
            }
        }
        if (i == 0) {
            reverse(nums.begin(), nums.end());
        }
    }
};

int main() {
    vector<int> nums = {2,3,1,3,3};
    Solution().nextPermutation(nums);
    for (auto n:nums)
    printf("%d ", n);
    printf("\n");
    return 0;
}