#include <vector>
#include <cstdio>

#include "helpers.h"

using namespace std;

class Solution {
public:
    TreeNode* searchBST(TreeNode* root, int val) {
        while (root && root->val != val) {
            root = root->val > val ? root->left : root->right;
        }
        return root;
    }
};

int main() {
    const int val = 2;
    vector<int> input{4,2,7,1,3};
    TreeNode *root = parse_tree(input);
    print_tree(Solution().searchBST(root, val));
    return 0;
}