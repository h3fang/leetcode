#include <stack>
#include <cstdio>

#include "helpers.h"

using namespace std;

class Solution {
public:
    int kthSmallest(TreeNode* root, int k) {
        stack<TreeNode*> s;

        TreeNode* r = root;
        while (true) {
            while (root) {
                s.push(root);
                root = root->left;
            }
            root = s.top();
            s.pop();
            k--;
            if (k == 0) {return root->val;}
            root = root->right;
        }
    }
};

int main() {
    vector<int> inputs = {3, 1, 4, null, 2};
    int k = 3;

    printf("%d\n", Solution().kthSmallest(parse_tree(inputs), k));
    return 0;
}