#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> binary(vector<int>& nums, int target, int a, int b) {
        if (a == b) {
            return nums[a] == target ? vector<int>{a,a} : vector<int>{-1,-1};
        }
        while (a<b) {
            int c = a + (b-a)/2;
            if (nums[c] < target) {
                a = c+1;
            } else if (nums[c] > target) {
                b = c-1;
            } else {
                auto l = binary(nums, target, a, c-1);
                auto r = binary(nums, target, c+1, b);
                return {l[0]==-1 ? c : l[0], r[1] == -1 ? c : r[1]};
            }
        }
        return nums[a] == target ? vector<int>{a,b} : vector<int>{-1,-1};
    }
    vector<int> searchRange(vector<int>& nums, int target) {
        const int N = nums.size();
        if (N == 0) {return {-1,-1};}
        return binary(nums, target, 0, N-1);
    }
};

int main() {
    vector<int> nums = {0, 1, 3, 4, 5, 5, 7};
    const int target = 5;
    auto r = Solution().searchRange(nums, target);
    printf("%d, %d\n", r[0], r[1]);
    return 0;
}