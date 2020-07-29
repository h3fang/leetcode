#include <cstdio>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        unordered_map<int, bool> seen;
        for (int n : nums) {
            if (seen[n]) {
                seen.erase(n);
            } else {
                seen[n] = true;
            }
        }

        auto it = seen.begin();
        return {it->first, (++it)->first};
    }
};

int main() {
    vector<int> nums = {1,2,1,3,2,5};
    printf("[");
    for (int n : Solution().singleNumber(nums)) {
        printf("%d,", n);
    }
    printf("\b]\n");
    return 0;
}