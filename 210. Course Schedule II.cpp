#include <cstdio>
#include <queue>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> findOrder(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> g(numCourses);
        vector<int> degree(numCourses);
        for (auto & p : prerequisites) {
            g[p[1]].push_back(p[0]);
            degree[p[0]]++;
        }
        queue<int> q;
        for (int i = 0; i < degree.size(); i++) {
            if (degree[i] == 0) {
                q.push(i);
            }
        }
        vector<int> r;
        while (!q.empty()) {
            int c = q.front();
            q.pop();
            r.push_back(c);
            numCourses--;
            for (int n : g[c]) {
                degree[n]--;
                if (degree[n] == 0) {
                    q.push(n);
                }
            }
        }
        if (numCourses == 0) {
            return r;
        } else {
            return {};
        }
    }
};

int main() {
    const int N = 3;
    vector<vector<int>> prerequisites = {{1, 0}, {0, 2}};
    auto r = Solution().findOrder(N, prerequisites);
    if (r.empty()) {
        printf("{}\n");
        return 0;
    }
    printf("{");
    for (int i : Solution().findOrder(N, prerequisites)) {
        printf("%d,", i);
    }
    printf("\b}\n");
    return 0;
}