#include <cassert>
#include <cstdio>
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
    vector<vector<int>> levelOrder(Node *root) {
        vector<vector<int>> r;
        deque<Node *> q;
        if (root) {
            q.push_back(root);
        }
        while (!q.empty()) {
            int level_size = q.size();
            vector<int> level;
            level.reserve(level_size);
            for (int i = 0; i < level_size; i++) {
                auto n = q.front();
                q.pop_front();
                level.push_back(n->val);
                for (auto child : n->children) {
                    q.push_back(child);
                }
            }
            r.push_back(level);
        }
        return r;
    }
};

Node *parse_nary_tree(const vector<int> nums) {
    if (nums.empty()) {
        return nullptr;
    }
    Node *root = new Node(nums[0]);
    deque<Node *> q;
    q.push_back(root);
    int i = 2;
    while (!q.empty()) {
        auto n = q.front();
        q.pop_front();
        while (i < nums.size() && nums[i] != null) {
            auto child = new Node(nums[i]);
            n->children.push_back(child);
            q.push_back(child);
            i += 1;
        }
        i += 1;
    }
    return root;
}

int main() {
    vector<int> nums = {1, null, 2,    3,    4,  5,    null, null, 6,  7,    null, 8, null,
                        9, 10,   null, null, 11, null, 12,   null, 13, null, null, 14};
    Node *root = parse_nary_tree(nums);
    auto r = Solution().levelOrder(root);
    vector<vector<int>> expected = {{1}, {2, 3, 4, 5}, {6, 7, 8, 9, 10}, {11, 12, 13}, {14}};
    assert(r == expected);
    return 0;
}