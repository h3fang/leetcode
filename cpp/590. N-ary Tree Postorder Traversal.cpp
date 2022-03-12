#include <algorithm>
#include <cassert>
#include <stack>
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
public:
    vector<int> postorder(Node *root) {
        vector<int> result;
        if (!root) {
            return result;
        }
        stack<Node *> s;
        s.push(root);
        while (!s.empty()) {
            auto n = s.top();
            s.pop();
            result.push_back(n->val);
            for (auto child : n->children) {
                s.push(child);
            }
        }
        reverse(result.begin(), result.end());
        return result;
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
    auto result = Solution().postorder(root);
    vector<int> expected = {5, 6, 3, 2, 4, 1};
    assert(result == expected);
    return 0;
}