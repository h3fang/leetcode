#include <cstdio>
#include <vector>
#include <queue>

#include "helpers.h"

using namespace std;

using Node = TreeNode;

class Solution {
public:
    Node* connect2(Node* root) {
        if (!root) {
            return root;
        }
        vector<Node *> level = {root};
        while (!level.empty()) {
            vector<Node *> next_level;
            Node* pre = nullptr;
            for (auto n : level) {
                if (n->left) {
                    next_level.push_back(n->left);
                }
                if (n->right) {
                    next_level.push_back(n->right);
                }
                if (pre) {
                    pre->next = n;
                }
                pre = n;
            }
            pre->next = nullptr;
            level = next_level;
        }
        return root;
    }

    Node* connect(Node* root) {
        if (!root) {
            return root;
        }
        queue<Node *> q;
        q.push(root);
        while (!q.empty()) {
            Node* pre = nullptr;
            auto m = q.size();
            for (int i=0; i<m; i++) {
                auto n = q.front();
                q.pop();
                if (n->left) {
                    q.push(n->left);
                }
                if (n->right) {
                    q.push(n->right);
                }
                if (pre) {
                    pre->next = n;
                }
                pre = n;
            }
        }
        return root;
    }
};

int main() {
    vector<int> nums = {1,2,3,4,5,6,7};
    auto root = Solution().connect(parse_tree(nums));
    print_tree(root);
    return 0;
}