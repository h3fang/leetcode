#include <cstdio>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int singleNumber(vector<int>& nums) {
        unordered_map<int, int> seen;
        for (int n : nums) {
            if (seen[n] == 2) {
                seen.erase(n);
            } else {
                seen[n]++;
            }
        }

        return seen.begin()->first;
    }
};

int main() {
    vector<int> nums = {2,2,3,2};
    printf("%d\n", Solution().singleNumber(nums));
    return 0;
}