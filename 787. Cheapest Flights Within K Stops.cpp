#include <cstdio>
#include <set>
#include <queue>
#include <vector>
#include <stack>
#include <limits>

using namespace std;

class Solution {
    struct NodeEntry {
        int node;
        int stops;
        int price;
        NodeEntry(int n, int s, int p) : node(n), stops(s), price(p) {}
        bool operator>(const NodeEntry& rhs) const {
            return price > rhs.price;
        }
    };

    typedef priority_queue<NodeEntry, vector<NodeEntry>, greater<NodeEntry>> pqueue;

public:
    int findCheapestPrice(int n, vector<vector<int>> &flights, int src, int dst, int K) {
        vector<vector<pair<int, int>>> adj_matrix(n);
        pqueue open;
        for (auto& f : flights) {
            adj_matrix[f[0]].push_back({f[1], f[2]});
        }
        open.emplace(src, 0, 0);

        while (!open.empty()) {
            auto ne = open.top();
            open.pop();

            if (ne.node == dst) {
                return ne.price;
            }

            if (ne.stops > K) {
                continue;
            }

            for (auto& edge : adj_matrix[ne.node]) {
                open.emplace(edge.first, ne.stops+1, ne.price + edge.second);
            }
        }
        return -1;
    }
};

int main() {
    const int n = 5, src = 0, dst = 2, k = 2;
    vector<vector<int>> edges = {{0,1,5},{1,2,5},{0,3,2},{3,1,2},{1,4,1},{4,2,1}};
    printf("%d\n", Solution().findCheapestPrice(n, edges, src, dst, k));
    return 0;
}