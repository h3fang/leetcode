#include <cstdio>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int singleNumber2(vector<int>& nums) {
        unordered_map<int, bool> seen;
        for (int n : nums) {
            if (seen[n]) {
                seen.erase(n);
            } else {
                seen[n] = true;
            }
        }

        return seen.begin()->first;
    }

    int singleNumber(vector<int>& nums) {
        int r = 0;
        for (int n : nums) {
            r ^= n;
        }

        return r;
    }
};

int main() {
    vector<int> nums = {2,2,1};
    printf("%d\n", Solution().singleNumber(nums));
    return 0;
}