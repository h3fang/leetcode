#include <algorithm>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    int twoCitySchedCost2(vector<vector<int>> &costs) {
        const int N = costs.size() / 2;
        nth_element(costs.begin(), costs.begin() + N, costs.end(),
                    [](const auto &l, const auto &r) { return (l[0] - l[1]) < (r[0] - r[1]); });
        int r = 0;
        for (int i = 0; i < N; i++) {
            r += costs[i][0] + costs[i + N][1];
        }
        return r;
    }

    int twoCitySchedCost(vector<vector<int>> &costs) {
        const int N = costs.size() / 2;
        sort(costs.begin(), costs.end(),
             [](const auto &l, const auto &r) { return abs(l[0] - l[1]) > abs(r[0] - r[1]); });
        int A = 0, B = 0, r = 0;
        for (int i = 0; i < N; i++) {
            const auto &c = costs[i];
            if (c[0] < c[1]) {
                r += c[0];
                A++;
            } else {
                r += c[1];
                B++;
            }
        }

        for (int i = N; i < 2 * N; i++) {
            const auto &c = costs[i];
            if (c[0] < c[1]) {
                if (A < N) {
                    r += c[0];
                    A++;
                } else {
                    r += c[1];
                    B++;
                }
            } else {
                if (B < N) {
                    r += c[1];
                    B++;
                } else {
                    r += c[0];
                    A++;
                }
            }
        }
        return r;
    }
};

int main() {
    vector<vector<int>> costs = {{10, 20}, {30, 200}, {400, 50}, {30, 20}};
    printf("%d\n", Solution().twoCitySchedCost2(costs));
    return 0;
}