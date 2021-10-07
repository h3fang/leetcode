#include <cstdio>
#include <vector>

#include "helpers.h"

using namespace std;

using Node = TreeNode;

class Solution {
public:
    Node* connect(Node* root) {
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
};

int main() {
    vector<int> nums = {1,2,3,4,5,6,7};
    auto root = Solution().connect(parse_tree(nums));
    print_tree(root);
    return 0;
}