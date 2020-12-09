#include <cmath>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int jump(vector<int> &nums) {
        const int N = nums.size();
        int steps = 0, start = 0, end = 0;
        while (end < N - 1) {
            steps++;
            int next_end = end;
            for (int i = start; i <= end; i++) {
                next_end = max(next_end, i + nums[i]);
            }
            start = end + 1;
            end = next_end;
        }
        return steps;
    }
};

int main() {
    vector<int> nums = {2, 3, 1, 1, 4};
    printf("%d\n", Solution().jump(nums));
    return 0;
}