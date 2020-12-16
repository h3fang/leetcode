#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    bool binary(vector<int>& nums, int target, int a, int b) {
        while (a<=b) {
            int c = a + (b-a)/2;
            if (nums[c] == target) {return true;}
            if (nums[a] < nums[b]) {
                if (nums[c] < target) {
                    a = c+1;
                } else {
                    b = c-1;
                }
            }
            else {
                if (binary(nums, target, a, c-1)) {
                    return true;
                } else {
                    return binary(nums, target, c+1, b);
                }
            }
        }
        return false;
    }

    bool search(vector<int>& nums, int target) {
        const int N = nums.size();
        if (N == 0) {return false;}
        if (N == 1) {return nums[0] == target;}
        return binary(nums, target, 0, nums.size()-1);
    }
};

int main() {
    vector<int> nums = {1,3,1,1,1};
    const int target = 3;
    printf("%s\n", Solution().search(nums, target) ? "true" : "false");
    return 0;
}