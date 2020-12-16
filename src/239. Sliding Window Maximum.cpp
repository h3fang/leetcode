#include <cstdio>
#include <queue>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> maxSlidingWindow(vector<int> &nums, int k) {
        vector<int> r;
        deque<int> q;
        for (int i = 0; i < nums.size(); i++) {
            if (!q.empty() && q.front() == i - k) {
                q.pop_front();
            }
            while (!q.empty() && nums[q.back()] < nums[i]) {
                q.pop_back();
            }
            q.push_back(i);
            if (i + 1 >= k) {
                r.push_back(nums[q.front()]);
            }
        }
        return r;
    }
};

int main() {
    vector<int> nums = {1, -1};
    const int k = 1;
    auto r = Solution().maxSlidingWindow(nums, k);
    for (int n : r) {
        printf("%d,", n);
    }
    printf("\b \n");
    return 0;
}
