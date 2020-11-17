#include <cstdio>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        const int N = nums.size();
        sort(nums.begin(), nums.end());
        vector<vector<int>> result = {nums};

        while(true) {
            int i = N - 1;
            for (; i > 0; i--) {
                if (nums[i-1] < nums[i]) {
                    int k = i;
                    for (int j = k; j < nums.size(); j++) {
                        if (nums[j] > nums[i-1]) {
                            k = j;
                        }
                    }
                    swap(nums[i-1], nums[k]);
                    reverse(nums.begin()+i, nums.end());
                    result.push_back(nums);
                    break;
                }
            }
            if (i == 0) {
                break;
            }
        }
        return result;
    }
};

int main() {
    vector<int> nums = {2,3,1,3,3};
    auto r = Solution().permuteUnique(nums);
    for (const auto& p : r) {
        for (int n : p) {
            printf("%d ", n);
        }
        printf("\b \n");
    }
    return 0;
}