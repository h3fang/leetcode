#include <cassert>
#include <climits>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    bool dfs(TreeNode *root, long lb, long rb) {
        if (!root) {
            return true;
        }
        if (root->val <= lb || root->val >= rb) {
            return false;
        }
        return dfs(root->left, lb, root->val) && dfs(root->right, root->val, rb);
    }

public:
    bool isValidBST(TreeNode *root) {
        return dfs(root, LONG_MIN, LONG_MAX);
    }
};

int main() {
    vector<int> nodes = {5, 1, 4, null, null, 3, 6};
    auto root = parse_tree(nodes);
    auto r = Solution().isValidBST(root);
    assert(r == false);
    return 0;
}
