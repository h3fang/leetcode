#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    void dfs(TreeNode *root, vector<int> &nums) {
        if (root) {
            dfs(root->left, nums);
            nums.push_back(root->val);
            dfs(root->right, nums);
        }
    }

public:
    vector<int> inorderTraversal(TreeNode *root) {
        vector<int> nums;
        dfs(root, nums);
        return nums;
    }
};

int main() {
    vector<int> nodes = {1, null, 2, 3};
    auto root = parse_tree(nodes);
    auto r = Solution().inorderTraversal(root);
    vector<int> expected = {1, 3, 2};
    assert(r == expected);
    return 0;
}
