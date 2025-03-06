#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    TreeNode *dfs(TreeNode *root) {
        auto left = root->left, right = root->right;
        root->left = nullptr;
        if (left) {
            auto l = dfs(left);
            root->right = left;
            root = l;
        }
        if (right) {
            auto r = dfs(right);
            root->right = right;
            root = r;
        }
        return root;
    }

public:
    void flatten(TreeNode *root) {
        if (root) {
            dfs(root);
        }
    }
};

int main() {
    auto r = parse_tree({1, 2, 5, 3, 4, null, 6});
    Solution().flatten(r);
    auto expected = parse_tree({1, null, 2, null, 3, null, 4, null, 5, null, 6});
    assert(equal(r, expected));
    return 0;
}
