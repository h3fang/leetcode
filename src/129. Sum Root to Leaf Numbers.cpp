#include <cstdio>
#include <vector>
#include "helpers.h"

using namespace std;

class Solution {
public:
    void dfs(TreeNode* root, int c, int& r) {
        if (!root) {return;}
        c = c*10 + root->val;
        if (!root->left && !root->right) {
            r += c;
            return;
        }
        if (root->left) { dfs(root->left, c, r); };
        if (root->right) { dfs(root->right, c, r); };
    }
    int sumNumbers(TreeNode* root) {
        int r = 0;
        dfs(root, 0, r);
        return r;
    }
};

int main() {
    vector<int> nums = {4,9,0,5,1};
    auto r = parse_tree(nums);
    printf("%d\n", Solution().sumNumbers(r));
    return 0;
}