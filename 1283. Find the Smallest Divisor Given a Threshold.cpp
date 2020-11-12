#include <cstdio>
#include <vector>
#include <cmath>

using namespace std;

class Solution {
public:
    int64_t result(vector<int> &nums, const int d) {
        int64_t r = 0;
        for (int n : nums) {
            r += n / d + ((n % d == 0) ? 0 : 1);
        }
        return r;
    }

    int smallestDivisor(vector<int> &nums, int threshold) {
        int left = 1, right = 2;
        while (result(nums, right) > threshold) {
            left = right;
            right = right << 1;
        }

        while (left <= right) {
            int d = (right - left) / 2 + left;
            int64_t r = result(nums, d);
            if (r > threshold) {
                left = d + 1;
            } else {
                right = d - 1;
            }
        }

        return left;
    }
};

int main() {
    vector<int> nums = {1,2,5,9};
    const int threadhold = 6;
    auto r = Solution().smallestDivisor(nums, threadhold);
    printf("%d\n", r);
    return 0;
}