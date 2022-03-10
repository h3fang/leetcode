#include <cassert>
#include <vector>

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
    vector<int> result;

public:
    vector<int> preorder(Node *root) {
        result.clear();
        dfs(root);
        return result;
    }
    void dfs(Node *root) {
        if (!root) {
            return;
        }
        result.push_back(root->val);
        for (auto child : root->children) {
            dfs(child);
        }
    }
};

const int null = __INT32_MAX__;

vector<Node *> parse_n_ary_tree(vector<int> &nodes, int &idx) {
    vector<Node *> result;
    for (; idx < nodes.size(); idx++) {
        if (nodes[idx] == null) {
            idx++;
            break;
        } else {
            result.push_back(new Node(nodes[idx]));
        }
    }
    for (auto r : result) {
        auto children = parse_n_ary_tree(nodes, idx);
        r->children = children;
    }
    return result;
}

Node *parse_n_ary_tree(vector<int> nodes) {
    int idx = 0;
    return parse_n_ary_tree(nodes, idx)[0];
}

int main() {
    auto root = parse_n_ary_tree({1, null, 3, 2, 4, null, 5, 6});
    auto result = Solution().preorder(root);
    vector<int> expected = {1, 3, 5, 6, 2, 4};
    assert(result == expected);
    return 0;
}