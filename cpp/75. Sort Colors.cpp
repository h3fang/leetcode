#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    void sortColors(vector<int>& nums) {
        int a = 0, b = nums.size() - 1;
        for (int i=0; i<=b; ) {
            if (nums[i] == 2) {
                swap(nums[i], nums[b--]);
            } else if (nums[i] == 0) {
                swap(nums[i++], nums[a++]);
            } else {
                i++;
            }
        }
    }
};

int main() {
    vector<int> nums = {1,2,0};
    Solution().sortColors(nums);
    for (int c : nums) {
        printf("%d ", c);
    }
    printf("\n");
    return 0;
}