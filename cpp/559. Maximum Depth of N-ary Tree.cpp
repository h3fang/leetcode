#include <cassert>
#include <deque>
#include <vector>

#include "helpers.h"

using namespace std;

class Node {
public:
    int val;
    vector<Node *> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node *> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
public:
    int maxDepth(Node *root) {
        if (!root) {
            return 0;
        }
        int r = 0;
        for (auto c : root->children) {
            r = max(r, maxDepth(c));
        }
        return r + 1;
    }
};

Node *parse_n_ary_tree(vector<int> &nodes) {
    Node *root = nullptr;
    int i = 0;
    if (nodes.empty()) {
        return root;
    } else {
        root = new Node(nodes[0]);
        i += 2;
    }
    deque<Node *> q{root};
    while (!q.empty()) {
        auto p = q.front();
        q.pop_front();
        while (i < nodes.size() && nodes[i] != null) {
            auto c = new Node(nodes[i]);
            p->children.push_back(c);
            q.push_back(c);
            i++;
        }
        i++;
    }
    return root;
}

int main() {
    auto nodes = vector<int>{1, null, 3, 2, 4, null, 5, 6};
    auto root = parse_n_ary_tree(nodes);
    assert(3 == Solution().maxDepth(root));
    return 0;
}
