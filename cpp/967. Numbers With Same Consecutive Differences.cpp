#include <cmath>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    // void dfs(vector<int> &ans, const int N, const int K, const int num) {
    //     if (N == 0) {
    //         ans.push_back(num);
    //         return;
    //     }

    //     const int digit = num % 10;
    //     int candidates[2] = {digit-K, digit+K};
    //     if (candidates[1] == candidates[0]) {
    //         candidates[1] = -1;
    //     }
    //     for (int d : candidates) {
    //         if (d >= 0 && d < 10) {
    //             dfs(ans, N - 1, K, num * 10 + d);
    //         }
    //     }
    // }

    // vector<int> numsSameConsecDiff(int N, int K) {
    //     vector<int> ans;
    //     for (int i = 1; i < 10; i++) {
    //         dfs(ans, N - 1, K, i);
    //     }
    //     return ans;
    // }

    vector<int> numsSameConsecDiff(int N, int K) {
        vector<int> level(9);
        for (int i = 1; i < 10; i++) {
            level[i-1] = i;
        }
        while (N > 1) {
            vector<int> next_level;
            for (int num : level) {
                const int digit = num % 10;
                int candidates[2] = {digit-K, digit+K};
                if (candidates[1] == candidates[0]) {
                    candidates[1] = -1;
                }
                for (int d : candidates) {
                    if (d >= 0 && d < 10) {
                        next_level.push_back(num * 10 + d);
                    }
                }
            }
            level = next_level;
            N--;
        }
        return level;
    }
};

int main() {
    auto r = Solution().numsSameConsecDiff(2, 1);
    if (r.empty()) {
        printf("[]\n");
    } else {
        printf("[");
        for (int n : r) {
            printf("%d,", n);
        }
        printf("\b]\n");
    }
    return 0;
}