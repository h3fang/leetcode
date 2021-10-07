#include <cstdio>
#include <queue>

#include "helpers.h"

using namespace std;

class Solution {
public:
    void insert(TreeNode* r, int val) {
        if (val < r->val) {
            if (r->left) {
                insert(r->left, val);
            }
            else {
                r->left = new TreeNode(val);
            }
        }
        else {
            if (r->right) {
                insert(r->right, val);
            }
            else {
                r->right = new TreeNode(val);
            }
        }
    }
    TreeNode* bstFromPreorder(vector<int>& preorder) {
        TreeNode* r = new TreeNode(preorder[0]);
        for (int i=1; i<preorder.size(); i++) {
            insert(r, preorder[i]);
        }
        return r;
    }
};

int main() {
    vector<int> inputs = {8,5,1,7,10,12};
    print_tree(Solution().bstFromPreorder(inputs));
    return 0;
}