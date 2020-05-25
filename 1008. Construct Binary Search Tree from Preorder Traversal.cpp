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
    auto r = Solution().bstFromPreorder(inputs);
    queue<TreeNode*> nodes;
    nodes.push(r);
    printf("%d ", r->val);
    while (!nodes.empty()) {
        auto n = nodes.front();
        nodes.pop();
        if (!n->left && !n->right) {
            continue;
        }
        if (!n->left) {
            printf("null ");
        }
        else {
            printf("%d ", n->left->val);
            nodes.push(n->left);
        }
        if (!n->right) {
            printf("null ");
        }
        else {
            printf("%d ", n->right->val);
            nodes.push(n->right);
        }
    }
    printf("\n");
    return 0;
}