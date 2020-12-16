#include <cstdio>
#include <vector>
#include <cmath>

using namespace std;

class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        const int N = nums.size();
        vector<int> r(N);
        int f = 0, b = N - 1, i = N - 1;
        while(i >= 0) {
            if (abs(nums[f]) < abs(nums[b])) {
                r[i--] = nums[b] * nums[b];
                b--;
            }
            else {
                r[i--] = nums[f] * nums[f];
                f++;
            }
        }
        return r;
    }
};

int main() {
    vector<int> nums = {-4,-1,0,3,10};
    for (int n : Solution().sortedSquares(nums)) {
        printf("%d,", n);
    }
    printf("\b \n");
    return 0;
}
