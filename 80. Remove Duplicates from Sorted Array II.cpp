#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int> &nums) {
        const int N = nums.size();
        if (N < 3) {
            return N;
        }
        bool twice = false;
        int prev = nums[0], end = 1;
        for (int i = 1; i < N; i++) {
            int n = nums[i];
            if (n == prev) {
                if (!twice) {
                    nums[end++] = n;
                    twice = true;
                }
            }
            else {
                twice = false;
                nums[end++] = n;
            }
            prev = n;
        }
        return end;
    }
};

int main() {
    vector<int> nums = {1, 1, 1, 2, 2, 3};
    printf("%d\n", Solution().removeDuplicates(nums));
    return 0;
}
