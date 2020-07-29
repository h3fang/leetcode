#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    bool binary(vector<int>& nums, int a, int b, int& m) {
        while (a<=b) {
            if (nums[a] < nums[b]) {
                m = nums[a];
                return true;
            }
            else {
                int c = a + (b-a)/2;
                int left, right;
                m = nums[c];
                if (binary(nums, a, c-1, left)) {
                    m = min(m, left);
                }
                if (binary(nums, c+1, b, right)){
                    m = min(m, right);
                }
                return true;
            }
        }
        return false;
    }
    int findMin(vector<int>& nums) {
        int m = 0;
        binary(nums, 0, nums.size()-1, m);
        return m;
    }
};

int main() {
    vector<int> nums = {3,1,3};
    printf("%d\n", Solution().findMin(nums));
    return 0;
}
