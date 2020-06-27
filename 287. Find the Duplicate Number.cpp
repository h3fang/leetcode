#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int findDuplicate2(vector<int>& nums) {
        vector<bool> s(nums.size());
        for (int n : nums) {
            if (s[n]) {
                return n;
            }
            s[n] = true;
        }
        return 0;
    }

    int findDuplicate(vector<int>& nums) {
        int hare = nums[0], tortoise = nums[0];
        do {
            hare = nums[nums[hare]];
            tortoise = nums[tortoise];
        }
        while (hare != tortoise);
        int intersection = tortoise;
        hare = nums[0];
        while (hare != tortoise) {
            hare = nums[hare];
            tortoise = nums[tortoise];
        }
        return tortoise;
    }
};

int main() {
    vector<int> nums = {1,3,4,2,2};
    printf("%d\n", Solution().findDuplicate(nums));
    return 0;
}