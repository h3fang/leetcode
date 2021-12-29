#include <cstdio>
#include <queue>
#include <vector>

#include "helpers.h"

using namespace std;

using Node = TreeNode;

class Solution {
public:
    Node *connect(Node *root) {
        if (!root) {
            return root;
        }
        queue<Node *> level;
        level.push(root);
        while (!level.empty()) {
            auto k = level.size();
            Node *pre = nullptr;
            for (auto i = 0; i < k; i++) {
                auto n = level.front();
                level.pop();
                if (n->left) {
                    level.push(n->left);
                }
                if (n->right) {
                    level.push(n->right);
                }
                if (pre) {
                    pre->next = n;
                }
                pre = n;
            }
            pre->next = nullptr;
        }
        return root;
    }
};

int main() {
    vector<int> nums = {1, 2, 3, 4, 5, 6, 7};
    auto root = Solution().connect(parse_tree(nums));
    print_tree(root);
    return 0;
}