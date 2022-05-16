#include "helpers.h"

#include <cassert>

class Solution {
public:
    TreeNode *inorderSuccessor(TreeNode *root, TreeNode *p) {
        TreeNode* result = nullptr;
        if (p->right) {
            result = p->right;
            while (result->left) {
                result = result->left;
            }
            return result;
        }
        while (root) {
            if (root->val > p->val) {
                result = root;
                root = root->left;
            } else {
                root = root->right;
            }
        }
        return result;
    }
};

TreeNode *find_node(TreeNode *root, int val) {
    if (!root || root->val == val) {
        return root;
    }
    auto l = find_node(root->left, val);
    if (l) {
        return l;
    }
    return find_node(root->right, val);
}

int main() {
    auto root = parse_tree({2, 1, 3});
    auto p = find_node(root, 1);
    auto result = Solution().inorderSuccessor(root, p);
    assert(2 == result->val);
    return 0;
}