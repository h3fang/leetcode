#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> ss(vector<int>& nums, int i) {
        if (i == nums.size()) {
            return {{}};
        }
        auto r = ss(nums, i+1);
        const int N = r.size();
        for (int j = 0; j < N; j++) {
            auto s = r[j];
            s.push_back(nums[i]);
            r.push_back(s);
        }
        return r;
    }
    vector<vector<int>> subsets(vector<int>& nums) {
        return ss(nums, 0);
    }
};

int main() {
    vector<int> nums = {1,2,3};
    printf("[\n");
    for (auto& s : Solution().subsets(nums)) {
        if (s.empty()) {
            printf("  [],\n");
            continue;
        }
        printf("  [");
        for (int i : s) {
            printf("%d,", i);
        }
        printf("\b],\n");
    }
    printf("]\n");
    return 0;
}