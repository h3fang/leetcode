#include <cstdio>
#include <stack>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    TreeNode *invertTree(TreeNode *root) {
        stack<TreeNode *> s;
        s.push(root);
        while (!s.empty()) {
            auto n = s.top();
            s.pop();
            if (n) {
                s.push(n->right);
                s.push(n->left);
                swap(n->left, n->right);
            }
        }
        return root;
    }
};

int main() {
    vector<int> input{4, 2, 7, 1, 3, 6, 9};
    TreeNode *root = parse_tree(input);
    print_tree(Solution().invertTree(root));
    return 0;
}