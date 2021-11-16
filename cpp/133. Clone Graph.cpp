#include <unordered_map>
#include <vector>
#include <cassert>
#include <algorithm>

using namespace std;

class Node {
public:
    int val;
    vector<Node *> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node *>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node *>();
    }
    Node(int _val, vector<Node *> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};

class Solution {
    unordered_map<Node *, Node *> map;

public:
    Node *cloneGraph(Node *node) {
        if (!node) {
            return nullptr;
        }
        Node *clone = new Node(node->val);
        map[node] = clone;
        for (auto next : node->neighbors) {
            if (map.find(next) != map.end()) {
                clone->neighbors.push_back(map[next]);
            } else {
                clone->neighbors.push_back(cloneGraph(next));
            }
        }
        return clone;
    }
};

int main(int argc, char const *argv[]) {
    vector<vector<int>> edges = {{2, 4}, {1, 3}, {2, 4}, {1, 3}};
    unordered_map<int, Node*> map;
    for (auto& e : edges) {
        if (map.find(e[0]) == map.end()) {
            map[e[0]] = new Node(e[0]);
        }
        if (map.find(e[1]) == map.end()) {
            map[e[1]] = new Node(e[1]);
        }
        map[e[0]]->neighbors.push_back(map[e[1]]);
        map[e[1]]->neighbors.push_back(map[e[0]]);
    }
    auto node = (*map.begin()).second;
    auto s = Solution();
    auto cloned = s.cloneGraph(node);

    assert(node != cloned && node->val == cloned->val);
    vector<int> children;
    for (auto n : node->neighbors) {
        children.push_back(n->val);
    }
    vector<int> children_cloned;
    for (auto n : cloned->neighbors) {
        children_cloned.push_back(n->val);
    }
    sort(children.begin(), children.end());
    sort(children_cloned.begin(), children_cloned.end());
    assert(children == children_cloned);
    return 0;
}
