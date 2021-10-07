#include <cstdio>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        const int N = nums.size();
        if (N == 0) { return 1; }
        for (int i=0; i<N; i++) {
            if (nums[i] > 0 && nums[i] <= N && nums[nums[i]-1] != nums[i]) {
                swap(nums[nums[i]-1], nums[i]);
                i--;
            }
        }
        for (int i=0; i<nums.size(); i++) {
            if (nums[i] != i+1) {
                return i+1;
            }
        }
        return nums.size() + 1;
    }
};

int main() {
    vector<int> nums = {3,4,-1,1};
    printf("%d\n", Solution().firstMissingPositive(nums));
    return 0;
}