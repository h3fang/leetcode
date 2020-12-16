#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        int a = 0, b = nums.size()-1;
        while (a <= b) {
            int c = a + (b-a)/2;
            if (nums[c] >= target) {
                b = c-1;
            } else {
                a = c+1;
            }
        }
        return a;
    }
};

int main() {
    vector<int> nums = {1,3,5,6};
    const int target = 2;
    printf("%d\n", Solution().searchInsert(nums, target));
    return 0;
}