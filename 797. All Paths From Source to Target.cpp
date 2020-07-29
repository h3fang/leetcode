#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    void bt(const vector<vector<int>> &graph, vector<vector<int>> &r, vector<int> &path, int c) {
        path.push_back(c);
        if (c == graph.size() - 1) {
            r.push_back(path);
            path.pop_back();
            return;
        }
        for (int n : graph[c]) {
            bt(graph, r, path, n);
        }
        path.pop_back();
    }
    vector<vector<int>> allPathsSourceTarget(vector<vector<int>> &graph) {
        vector<vector<int>> r;
        vector<int> path;
        bt(graph, r, path, 0);
        return r;
    }
};

int main() {
    vector<vector<int>> graph = {{1, 2}, {3}, {3}, {}};
    auto r = Solution().allPathsSourceTarget(graph);
    printf("[");
    if (r.empty()) {
        printf("]\n");
    } else {
        for (const auto &path : Solution().allPathsSourceTarget(graph)) {
            printf("[");
            for (int n : path) {
                printf("%d,", n);
            }
            printf("\b],");
        }
        printf("\b]\n");
    }
    return 0;
}
