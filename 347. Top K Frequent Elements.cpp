#include <cstdio>
#include <vector>
#include <unordered_map>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> freq;
        for (int i : nums) {
            freq[i]++;
        }
        vector<pair<int, int>> fs;
        for (auto & p : freq) {
            fs.push_back(p);
        }
        nth_element(fs.begin(), fs.begin()+k, fs.end(), [](const auto &l, const auto & r){ return l.second > r.second; });
        vector<int> r;
        for (auto it = fs.begin(); it < fs.begin() + k; it++) {
            r.push_back((*it).first);
        }
        return r;
    }
};

int main() {
    vector<int> nums = {1,1,1,2,2,3};
    const int k = 2;
    printf("[");
    for (int i : Solution().topKFrequent(nums, k)) {
        printf("%d,", i);
    }
    printf("\b]\n");
    return 0;
}