#include <unordered_map>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
  public:
    int findMaxLength(vector<int> &nums) {
        int r = 0, sum = 0;
        unordered_map<int, int> m;

        for (int i = 0; i < nums.size(); i++) {
            sum += (nums[i] == 0) ? -1 : 1;

            if (sum == 0) {
                r = max(r, i + 1);
            } else {
                auto it = m.find(sum);
                if (it != m.end()) {
                    r = max(r, i - it->second);
                } else if (it == m.end()) {
                    m.insert({sum, i});
                }
            }
        }
        return r;
    }
};

int main() {
    vector<int> nums = {0, 1, 0, 1, 1, 1, 0};
    printf("%d\n", Solution().findMaxLength(nums));
    return 0;
}