#include <cstdio>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    void bt(vector<int>& candidates, int target, vector<int>& candidate, vector<vector<int>>& r, int first_candidate) {
        if (target == 0) {
            r.push_back(candidate);
        }

        int last_candidate = -1;
        for (int i=first_candidate; i<candidates.size(); i++) {
            if (last_candidate == candidates[i]) {
                continue;
            }
            last_candidate = candidates[i];
            if (target >= candidates[i]) {
                candidate.push_back(candidates[i]);
                bt(candidates, target-candidates[i], candidate, r, i+1);
                candidate.pop_back();
            }
        }
    }

    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        const int N = candidates.size();
        vector<vector<int>> r;
        vector<int> candidate;
        bt(candidates, target, candidate, r, 0);
        return r;
    }
};

int main() {
    vector<int> candidates = {2,5,2,1,2};
    const int target = 5;
    for (auto & s : Solution().combinationSum2(candidates, target)) {
        for (int n : s) {
            printf("%d ", n);
        }
        printf("\n");
    }
    return 0;
}