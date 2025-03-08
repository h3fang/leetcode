#include <algorithm>
#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> topKFrequent(vector<int> &nums, int k) {
        unordered_map<int, int> freq;
        for (int i : nums) {
            freq[i]++;
        }
        vector<pair<int, int>> fs;
        for (auto &p : freq) {
            fs.push_back(p);
        }
        nth_element(fs.begin(), fs.begin() + k, fs.end(),
                    [](auto &a, auto &b) { return a.second > b.second; });
        vector<int> r;
        for (int i = 0; i < k; i++) {
            r.push_back(fs[i].first);
        }
        return r;
    }
};

int main() {
    vector<int> expected = {1, 2};
    sort(expected.begin(), expected.end());

    vector<int> nums = {1, 1, 1, 2, 2, 3};
    auto r = Solution().topKFrequent(nums, 2);
    sort(r.begin(), r.end());

    assert(r == expected);
    return 0;
}
