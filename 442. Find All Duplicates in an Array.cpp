#include <cstdio>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    vector<int> findDuplicates(vector<int>& nums) {
        vector<int> r;
        unordered_map<int, bool> m;
        for (int n : nums) {
            if (m[n]) {
                r.push_back(n);
            } else {
                m[n] = true;
            }
        }
        return r;
    }

    vector<int> findDuplicates2(vector<int>& nums) {
        vector<int> r;
        for (int i = 0; i < nums.size(); i++) {
            while ((i != nums[i] - 1) && nums[nums[i]-1] != nums[i]) {
                swap(nums[i], nums[nums[i]-1]);
            }
        }
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i]-1 != i) {
                r.push_back(nums[i]);
            }
        }
        return r;
    }
};

int main() {
    vector<int> nums = {4, 3, 2, 7, 8, 2, 3, 1};
    auto r = Solution().findDuplicates2(nums);
    if (r.empty()) {
        printf("[]\n");
    } else {
        printf("[");
        for (int i : r) {
            printf("%d,", i);
        }
        printf("\b]\n");
    }
    return 0;
}