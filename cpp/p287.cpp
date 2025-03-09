#include <cassert>
#include <vector>

using namespace std;

class Solution {
public:
    int findDuplicate(vector<int> &nums) {
        int hare = nums[0], tortoise = nums[0];
        do {
            hare = nums[nums[hare]];
            tortoise = nums[tortoise];
        } while (hare != tortoise);
        hare = nums[0];
        while (hare != tortoise) {
            hare = nums[hare];
            tortoise = nums[tortoise];
        }
        return tortoise;
    }
};

int main() {
    vector<int> nums = {1, 3, 4, 2, 2};
    assert(2 == Solution().findDuplicate(nums));
    return 0;
}