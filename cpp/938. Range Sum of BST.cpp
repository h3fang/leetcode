#include <cstdio>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    int rangeSumBST(TreeNode* root, int low, int high) {
        if (!root) {
            return 0;
        }
        if (root->val > high) {
            return rangeSumBST(root->left, low, high);
        }
        else if (root->val < low) {
            return rangeSumBST(root->right, low, high);
        }
        else {
            return root->val + rangeSumBST(root->left, low, high) + rangeSumBST(root->right, low, high);
        }
    }
};

int main() {
    auto root = parse_tree({10,5,15,3,7,13,18,1,null,6});
    printf("%d\n", Solution().rangeSumBST(root, 6, 10));
    return 0;
}
