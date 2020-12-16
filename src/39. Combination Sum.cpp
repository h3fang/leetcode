#include <cstdio>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        const int N = candidates.size();
        vector<vector<vector<int>>> dp(target+1);
        dp[0] = {{}};
        for (auto c : candidates) {
            for (int i = c; i <= target; i++) {
                for (auto s : dp[i-c]) {
                    s.push_back(c);
                    dp[i].push_back(s);
                }
            }
        }
        return dp[target];
    }

    void bt(vector<int>& candidates, int target, vector<int>& candidate, vector<vector<int>>& r, int first_candidate) {
        if (target == 0) {
            r.push_back(candidate);
        }

        for (int i=first_candidate; i<candidates.size(); i++) {
            if (target >= candidates[i]) {
                candidate.push_back(candidates[i]);
                bt(candidates, target-candidates[i], candidate, r, i);
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
    vector<int> candidates = {2,3,5};
    const int target = 10;
    for (auto & s : Solution().combinationSum2(candidates, target)) {
        for (int n : s) {
            printf("%d ", n);
        }
        printf("\n");
    }
    return 0;
}