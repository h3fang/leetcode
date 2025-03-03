#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> maxSlidingWindow(vector<int> &nums, int k) {
        vector<int> r;
        deque<int> q;
        for (int i = 0; i < nums.size(); i++) {
            while (!q.empty() && q.front() <= i - k) {
                q.pop_front();
            }
            while (!q.empty() && nums[q.back()] <= nums[i]) {
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
    vector<int> nums = {1, 3, -1, -3, 5, 3, 6, 7};
    const int k = 3;
    auto r = Solution().maxSlidingWindow(nums, k);
    vector<int> expected = {3, 3, 5, 5, 6, 7};
    assert(r == expected);
    return 0;
}
