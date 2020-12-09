#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    void bt(vector<int>& nums, int k, vector<vector<int>>& r) {
        if (k == nums.size()) {
            r.push_back(nums);
        }

        for (int i = k; i < nums.size(); i++) {
            swap(nums[i], nums[k]);
            bt(nums, k+1, r);
            swap(nums[i], nums[k]);
        }
    }
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> r;
        bt(nums, 0, r);
        return r;
    }
};

int main() {
    vector<int> nums = {1,2,3};
    for (const auto& p : Solution().permute(nums)) {
        for (int n : p) {
            printf("%d,", n);
        }
        printf("\b \n");
    }
    return 0;
}