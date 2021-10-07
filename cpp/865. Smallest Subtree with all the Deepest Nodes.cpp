#include <cstdio>
#include <vector>
// #include <unordered_map>

#include "helpers.h"

using namespace std;

class Solution {
    // unordered_map<TreeNode*, int> depth;
    // int max_depth = 0;

public:
    // void dfs_depth(TreeNode* root, const int d) {
    //     if (!root) {
    //         return;
    //     }
    //     depth[root] = d+1;
    //     max_depth = max(max_depth, d+1);
    //     dfs_depth(root->left, d+1);
    //     dfs_depth(root->right, d+1);
    // }

    // TreeNode* dfs(TreeNode* root) {
    //     if (!root || depth[root] == max_depth) {
    //         return root;
    //     }
    //     if (depth[root->left] == max_depth && depth[root->right] == max_depth) {
    //         return root;
    //     }
    //     TreeNode *left = dfs(root->left), *right = dfs(root->right);
    //     if (left && right) {
    //         return root;
    //     }
    //     return left ? left : right;
    // }

    // TreeNode* subtreeWithAllDeepest(TreeNode* root) {
    //     depth[nullptr] = 0;
    //     dfs_depth(root, -1);
    //     return dfs(root);
    // }

    pair<TreeNode*, int> dfs(TreeNode* root) {
        if (!root || depth[root] == max_depth) {
            return {nullptr
            };
        }
        if (depth[root->left] == max_depth && depth[root->right] == max_depth) {
            return root;
        }
        TreeNode *left = dfs(root->left), *right = dfs(root->right);
        if (left && right) {
            return root;
        }
        return left ? left : right;
    }

    TreeNode* subtreeWithAllDeepest(TreeNode* root) {
        depth[nullptr] = 0;
        dfs(root, -1);
        return dfs(root);
    }
};

int main() {
    vector<int> nodes = {3,5,1,6,2,0,8,null,null,7,4};
    auto root = parse_tree(nodes);
    print_tree(Solution().subtreeWithAllDeepest(root));
    return 0;
}
