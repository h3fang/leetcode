#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    int singleNumber(vector<int> &nums) {
        int r = 0;
        for (int x : nums) {
            r ^= x;
        }
        return r;
    }
};

int main() {
    vector<int> nums = {2, 2, 3};
    assert(3 == Solution().singleNumber(nums));
    return 0;
}