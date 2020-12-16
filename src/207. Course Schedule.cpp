#include <cstdio>
#include <queue>
#include <vector>

using namespace std;

class Solution {
public:
    bool dfs(vector<bool> &visited, int n, vector<vector<int>> &g, vector<bool> &checking) {
        if (checking[n]) { return false; }
        if (visited[n]) { return true; }
        checking[n] = true;
        for (auto &&e : g[n]) {
            if (!visited[e] && !dfs(visited, e, g, checking)) { return false; }
        }
        checking[n] = false;
        visited[n] = true;
        return true;
    }

    bool canFinish2(int numCourses, vector<vector<int>> &prerequisites) {
        vector<vector<int>> g(numCourses);
        for (auto &p : prerequisites) {
            g[p[1]].push_back(p[0]);
        }
        vector<bool> visited(numCourses, false);
        for (int i = 0; i < numCourses; i++) {
            vector<bool> checking(numCourses, false);
            if (!visited[i] && !dfs(visited, i, g, checking)) { return false; }
        }

        return true;
    }

    bool canFinish(int numCourses, vector<vector<int>> &prerequisites) {
        vector<vector<int>> g(numCourses);
        vector<int> degree(numCourses, 0);
        for (auto &p : prerequisites) {
            g[p[1]].push_back(p[0]);
            degree[p[0]]++;
        }
        queue<int> q;
        for (int i = 0; i < numCourses; i++)
            if (degree[i] == 0) q.push(i);
        while (!q.empty()) {
            int curr = q.front();
            q.pop();
            numCourses--;
            for (auto next : g[curr])
                if (--degree[next] == 0) { q.push(next); }
        }
        return numCourses == 0;
    }
};

int main() {
    const int N = 3;
    vector<vector<int>> prerequisites = {{1, 0}, {0, 1}};
    printf("%d\n", Solution().canFinish2(N, prerequisites));
    return 0;
}