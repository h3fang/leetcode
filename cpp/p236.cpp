#include <cassert>
#include <tuple>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    TreeNode *node = nullptr;
    tuple<bool, bool> dfs(TreeNode *root, TreeNode *p, TreeNode *q) {
        if (!root) {
            return {false, false};
        }
        bool a = p == root, b = q == root;
        auto l = dfs(root->left, p, q);
        a = a || get<0>(l);
        b = b || get<1>(l);
        if (a && b && !node) {
            node = root;
            return {a, b};
        }

        auto r = dfs(root->right, p, q);
        a = a || get<0>(r);
        b = b || get<1>(r);
        if (a && b && !node) {
            node = root;
        }
        return {a, b};
    }

public:
    TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
        dfs(root, p, q);
        return node;
    }
};

int main() {
    auto r = parse_tree({3, 5, 1, 6, 2, 0, 8, null, null, 7, 4});
    auto p = find_node(r, 5);
    auto q = find_node(r, 1);
    auto ans = Solution().lowestCommonAncestor(r, p, q);
    assert(ans && ans->val == 3);
    return 0;
}
