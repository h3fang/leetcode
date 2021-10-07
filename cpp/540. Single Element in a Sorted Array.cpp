#include <vector>
#include <map>
#include <algorithm>
#include <cstdio>

using namespace std;

class Solution {
public:
    int singleNonDuplicate(vector<int>& nums) {
        int a=0, b=nums.size()-1, c = (a+b)/2;
        while (a < b) {
            if (c%2==0) {
                if (nums[c+1] == nums[c]) {
                    a = c+2;
                }
                else if (nums[c-1] == nums[c]) {
                    b = c-2;
                }
                else {
                    return nums[c];
                }
            }
            else {
                if (nums[c+1] == nums[c]) {
                    b = c-1;
                }
                else if (nums[c-1] == nums[c]) {
                    a = c+1;
                }
                else {
                    return nums[c];
                }
            }
            c = (a+b)/2;
        }
        return nums[c];
    }
};

int main() {
    vector<int> nums = {1,1,2,2,3,3,4,8,8};

    auto s = Solution().singleNonDuplicate(nums);
    printf("%d\n",s);

    return 0;
}