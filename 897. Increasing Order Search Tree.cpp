#include <cstdio>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    TreeNode* prev = nullptr;

public:
    TreeNode* increasingBST(TreeNode* root) {
        vector<TreeNode*> sorted;
        TreeNode dummy;
        prev = &dummy;
        dfs(root);
        return dummy.right;
    }

    void dfs(TreeNode* root) {
        if (root) {
            dfs(root->left);
            root->left = nullptr;
            prev->right = root;
            prev = root;
            dfs(root->right);
        }
    }
};

int main() {
    vector<int> nodes = {5,3,6,2,4,null,8,1,null,null,null,7,9};
    auto root = parse_tree(nodes);
    print_tree(Solution().increasingBST(root));
    return 0;
}
