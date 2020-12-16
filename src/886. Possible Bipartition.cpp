#include <set>
#include <queue>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    bool checkNeighbors(const vector<vector<int>>& g, int node, vector<int>& colors, int color) {
        if (colors[node] != 0) {
            return colors[node] == color;
        }
        colors[node] = color;
        for (int n : g[node]) {
            if (!checkNeighbors(g, n, colors, color==1 ? 2 : 1)) {
                return false;
            }
        }
        return true;
    }

    bool possibleBipartition(int N, vector<vector<int>>& dislikes) {
        const int M = dislikes.size();
        vector<vector<int>> g(N+1);
        vector<int> colors(N+1);
        for (const auto& d : dislikes) {
            g[d[0]].push_back(d[1]);
            g[d[1]].push_back(d[0]);
        }
        for (int n=1; n < g.size(); n++) {
            if (colors[n] == 0 && !checkNeighbors(g, n, colors, 1)) {
                return false;
            }
        }
        return true;
    }
};

int main() {
    // const int N = 3;
    // vector<vector<int>> dislikes = {{1,2},{1,3}};
    const int N = 10;
    vector<vector<int>> dislikes = {{4,7},{4,8},{2,8},{8,9},{1,6},{5,8},{1,2},{6,7},{3,10},{8,10},{1,5},{7,10},{1,10},{3,5},{3,6},{1,4},{3,9},{2,3},{1,9},{7,9},{2,7},{6,8},{5,7},{3,4}};
    // const int N = 5;
    // vector<vector<int>> dislikes = {{1,2},{3,4},{4,5},{3,5}};
    printf("%d\n", Solution().possibleBipartition(N, dislikes));
    return 0;
}