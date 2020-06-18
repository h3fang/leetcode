#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int binary(vector<int>& nums, int target, int a, int b) {
        while (a<=b) {
            int c = a + (b-a)/2;
            if (nums[c] == target) {return c;}
            if (nums[a] <= nums[b]) {
                if (nums[c] < target) {
                    a = c+1;
                } else {
                    b = c-1;
                }
            }
            else {
                int left = binary(nums, target, a, c-1);
                if (left != -1) {
                    return left;
                } else {
                    return binary(nums, target, c+1, b);
                }
            }
        }
        return -1;
    }
    int search(vector<int>& nums, int target) {
        const int N = nums.size();
        if (N == 0) {return -1;}
        if (N == 1) {return nums[0] == target ? 0 : -1;}
        return binary(nums, target, 0, nums.size()-1);
    }
};

int main() {
    vector<int> nums = {4, 5, 6, 7, 1, 2, 3};
    const int target = 1;
    printf("%d\n", Solution().search(nums, target));
    return 0;
}